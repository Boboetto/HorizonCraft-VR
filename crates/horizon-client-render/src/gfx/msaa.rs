//! VR's multisampling: the world pass's first part (sky, solid terrain, block entities and
//! entities) draws into multisampled colour and depth targets, its colour resolved into the
//! world target at the end of that part, its depth copied (sample 0) into the world's depth
//! for the rest of the world pass (translucent terrain, particles, clouds, weather...),
//! which draws on single-sampled as before. Only frames whose world pass is that one plain
//! pass take it (`Gfx::render_screen`'s `msaa_frame`).
use super::SolidPipeline;

const COPY_SHADER: &str = r"
@group(0) @binding(0) var depth_ms: texture_depth_multisampled_2d;

@vertex
fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let uv = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs(@builtin(position) p: vec4<f32>) -> @builtin(frag_depth) f32 {
    return textureLoad(depth_ms, vec2<i32>(p.xy), 0);
}
";

/// The multisampled copies of the first part's pipelines.
pub(super) struct Pipelines {
    pub sky_base: wgpu::RenderPipeline,
    pub sunrise: wgpu::RenderPipeline,
    pub dark_disc: wgpu::RenderPipeline,
    pub celestial: wgpu::RenderPipeline,
    pub static_entity: wgpu::RenderPipeline,
    pub entity: wgpu::RenderPipeline,
    pub solid: SolidPipeline,
    pub solid_opaque: SolidPipeline,
    pub solid_cutout: SolidPipeline,
}

struct Targets {
    size: [u32; 2],
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    group: wgpu::BindGroup,
}

pub(super) struct Msaa {
    pub samples: u32,
    pub pipelines: Pipelines,
    copy: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    targets: Option<Targets>,
}

impl Msaa {
    pub(super) fn new(device: &wgpu::Device, samples: u32, pipelines: Pipelines) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            source: wgpu::ShaderSource::Wgsl(COPY_SHADER.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: true,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let copy = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { samples, pipelines, copy, layout, targets: None }
    }

    /// Targets the size of the world's (`size`), in its colour `format`.
    pub(super) fn fit(&mut self, device: &wgpu::Device, size: [u32; 2], format: wgpu::TextureFormat) {
        if self.targets.as_ref().is_some_and(|t| t.size == size && t.format == format) {
            return;
        }
        let texture = |label, format, usage| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: self.samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&wgpu::TextureViewDescriptor::default())
        };
        let color = texture("Horizon VR MSAA colour", format, wgpu::TextureUsages::RENDER_ATTACHMENT);
        let depth = texture(
            "Horizon VR MSAA depth",
            wgpu::TextureFormat::Depth32Float,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&depth) }],
        });
        self.targets = Some(Targets { size, format, color, depth, group });
    }

    /// The multisampled colour and depth (after `fit`).
    pub(super) fn views(&self) -> Option<(&wgpu::TextureView, &wgpu::TextureView)> {
        self.targets.as_ref().map(|t| (&t.color, &t.depth))
    }

    /// The multisampled depth's sample 0 into the world's `depth`.
    pub(super) fn copy_depth(&self, encoder: &mut wgpu::CommandEncoder, depth: &wgpu::TextureView) {
        let Some(t) = &self.targets else { return };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Horizon VR MSAA depth copy"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.copy);
        pass.set_bind_group(0, &t.group, &[]);
        pass.draw(0..3, 0..1);
    }
}
