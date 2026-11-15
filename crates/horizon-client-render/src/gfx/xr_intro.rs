//! VR: the intro shown once, when the title screen first appears. In the dark, the Mojang
//! Studios logo (the player's own resources' sprite, as the loading screen draws it) floats
//! far ahead with a red thickness behind it, comes at the viewer through a tunnel of pixel
//! stars and passes; the title's panorama then rises out of the dark and the title's panel
//! flies in from afar to its place. The viewer never moves (the things do), and any button
//! skips to the arrival. Drawn into each eye's image where the menu's panel is drawn.
use glam::{Mat4, Quat, Vec3};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// When the logo starts its rush, when the panorama starts to show, when the panel starts
/// its flight, and the end, in seconds.
const RUSH: f32 = 2.6;
const DAWN: f32 = 3.3;
const ARRIVE: f32 = 3.7;
const END: f32 = 5.8;

struct State {
    /// The logo's sprite in the HUD atlas: u0, v0, u1, v1.
    logo: Option<[f32; 4]>,
    asked: bool,
    start: Option<Instant>,
    done: bool,
}

static STATE: Mutex<State> = Mutex::new(State { logo: None, asked: false, start: None, done: false });

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The game, each frame its title screen shows: the intro plays once from the first of them.
/// `logo`: the Mojang Studios sprite's place in the HUD atlas (u0, v0, u1, v1).
pub fn request(logo: Option<[f32; 4]>) {
    let mut s = state();
    s.asked = true;
    if logo.is_some() {
        s.logo = logo;
    }
}

/// The intro's time in seconds while it plays (it starts at the first call after `request`).
pub(super) fn time() -> Option<f32> {
    let mut s = state();
    if !s.asked || s.done || std::env::var("HORIZON_XR_INTRO").as_deref() == Ok("off") {
        return None;
    }
    let start = *s.start.get_or_insert_with(Instant::now);
    let t = start.elapsed().as_secs_f32();
    if t >= END {
        s.done = true;
        return None;
    }
    Some(t)
}

/// A button: on to the arrival (the panorama and the panel coming in).
pub(super) fn skip() {
    let mut s = state();
    if let Some(start) = s.start {
        let t = start.elapsed().as_secs_f32();
        if t > 0.4 && t < DAWN {
            s.start = Some(Instant::now() - Duration::from_secs_f32(DAWN));
        }
    }
}

/// The title screen is gone (a world opens): no intro any more.
pub(super) fn end() {
    let mut s = state();
    if s.start.is_some() {
        s.done = true;
    }
}

fn logo() -> Option<[f32; 4]> {
    state().logo
}

const SHADER: &str = r"
@group(0) @binding(0) var atlas: texture_2d<f32>;
@group(0) @binding(1) var panel: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct In {
    @builtin(vertex_index) i: u32,
    @location(0) m0: vec4<f32>,
    @location(1) m1: vec4<f32>,
    @location(2) m2: vec4<f32>,
    @location(3) m3: vec4<f32>,
    @location(4) uv: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(6) mode: vec4<f32>,
};

struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) mode: f32,
};

@vertex
fn vs(in: In) -> Out {
    let corner = vec2<f32>(f32(in.i & 1u), f32((in.i >> 1u) & 1u));
    var out: Out;
    out.position = mat4x4<f32>(in.m0, in.m1, in.m2, in.m3) * vec4<f32>(corner.x - 0.5, 0.5 - corner.y, 0.0, 1.0);
    out.uv = mix(in.uv.xy, in.uv.zw, corner);
    out.color = in.color;
    out.mode = in.mode.x;
    return out;
}

// Mode 0: the colour. 1: the atlas's texel (the logo: white, its alpha its shape) times the
// colour. 2: the menu panel's image, what is (nearly) opaque of it, at the colour's alpha.
@fragment
fn fs(in: Out) -> @location(0) vec4<f32> {
    let a = textureSample(atlas, samp, in.uv);
    let p = textureSample(panel, samp, in.uv);
    if in.mode < 0.5 {
        return in.color;
    }
    if in.mode < 1.5 {
        return vec4<f32>(in.color.rgb * a.rgb, in.color.a * a.a);
    }
    if p.a < 0.9 {
        discard;
    }
    return vec4<f32>(p.rgb, in.color.a);
}
";

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Quad {
    mvp: [f32; 16],
    uv: [f32; 4],
    color: [f32; 4],
    mode: [f32; 4],
}

const MAX_QUADS: usize = 256;
const STARS: u32 = 150;

pub(super) struct XrIntro {
    format: wgpu::TextureFormat,
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    quads: wgpu::Buffer,
}

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A number in 0..1 from `i` and `salt`, the same every frame.
fn hash(i: u32, salt: u32) -> f32 {
    let mut x = i.wrapping_mul(0x9E37_79B9) ^ salt.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    x = x.wrapping_mul(0x297A_2D39);
    x ^= x >> 15;
    (x >> 8) as f32 / (1u32 << 24) as f32
}

/// How far the stars have come by `t`, in metres: slow, a rush as the logo passes, slow again.
fn travelled(t: f32) -> f32 {
    let speed = |t: f32| 0.6 + 30.0 * smooth(RUSH - 0.3, RUSH + 0.7, t) * (1.0 - smooth(ARRIVE, ARRIVE + 1.3, t));
    let steps = 64;
    let dt = t / steps as f32;
    (0..steps).map(|k| speed((k as f32 + 0.5) * dt) * dt).sum()
}

impl XrIntro {
    pub(super) fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Horizon VR intro"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let texture = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Horizon VR intro"),
            entries: &[
                texture(0),
                texture(1),
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Horizon VR intro"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let attributes = wgpu::vertex_attr_array![
            0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4,
            4 => Float32x4, 5 => Float32x4, 6 => Float32x4
        ];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Horizon VR intro"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Quad>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &attributes,
                })],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Horizon VR intro"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let quads = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Horizon VR intro"),
            size: (MAX_QUADS * std::mem::size_of::<Quad>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { format, pipeline, layout, sampler, quads }
    }

    pub(super) fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    /// The intro at `t` seconds into `target`, seen through `view_proj` (camera-relative).
    /// `panel`: the menu panel's rotation, middle and size, the stage it plays on; `image`
    /// its picture, `atlas` the HUD's sprites (the logo's).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        atlas: &wgpu::Texture,
        image: &wgpu::Texture,
        view_proj: Mat4,
        (rotation, middle, size): (Quat, Vec3, [f32; 2]),
        t: f32,
    ) {
        let mut list: Vec<Quad> = Vec::with_capacity(MAX_QUADS);
        // Toward the viewer from the panel, and the panel's right and up.
        let mut toward = rotation * Vec3::Z;
        if toward.dot(-middle) < 0.0 {
            toward = -toward;
        }
        // A quad `w` x `h` facing the viewer, `d` metres toward them from the panel's plane
        // and `x`, `y` across it.
        let mut quad = |d: f32, x: f32, y: f32, w: f32, h: f32, uv: [f32; 4], color: [f32; 4], mode: f32| {
            let at = middle + toward * d + rotation * Vec3::new(x, y, 0.0);
            let model = Mat4::from_scale_rotation_translation(Vec3::new(w, h, 1.0), rotation, at);
            list.push(Quad {
                mvp: (view_proj * model).to_cols_array(),
                uv,
                color,
                mode: [mode, 0.0, 0.0, 0.0],
            });
        };
        let rush = ((t - RUSH) / (ARRIVE - RUSH)).clamp(0.0, 1.0);
        // The stars: a tunnel about the way to the panel, each coming from 30 m beyond it to
        // behind the viewer, again and again.
        let glow = smooth(0.0, 0.6, t) * (1.0 - smooth(ARRIVE + 0.7, END - 0.4, t));
        let come = travelled(t);
        if glow > 0.0 {
            for i in 0..STARS {
                let angle = hash(i, 1) * std::f32::consts::TAU;
                let radius = 1.3 + 6.0 * hash(i, 2).powf(0.7);
                let d = (hash(i, 3) * 40.0 + come).rem_euclid(40.0) - 30.0;
                let side = 0.04 + 0.08 * hash(i, 4);
                let alpha = glow * smooth(-30.0, -22.0, d) * (0.5 + 0.5 * hash(i, 5));
                // One in six in the logo's red.
                let color = if i % 6 == 0 { [0.94, 0.2, 0.24, alpha] } else { [1.0, 1.0, 1.0, alpha] };
                quad(d, radius * angle.cos(), radius * angle.sin(), side, side, [0.0; 4], color, 0.0);
            }
        }
        // The logo: its two halves side by side (the sprite's top half is the left one, its
        // bottom half the right one), 5 m wide, with a red thickness behind its white face.
        if let Some([u0, v0, u1, v1]) = logo()
            && t < ARRIVE
        {
            let d = -6.0 + 3.0 * smooth(0.0, RUSH, t) + 9.0 * rush * rush * rush;
            let alpha = smooth(0.1, 0.8, t) * (1.0 - smooth(0.55, 0.95, rush));
            let (width, height) = (5.0, 1.25);
            let inset = (u1 - u0) * 0.002;
            let vm = (v0 + v1) * 0.5;
            let layers = 10;
            for k in (0..layers).rev() {
                let depth = k as f32 / (layers - 1) as f32;
                let color = if k == 0 {
                    [1.0, 1.0, 1.0, alpha]
                } else {
                    [0.94 - 0.55 * depth, 0.2 - 0.14 * depth, 0.24 - 0.16 * depth, alpha]
                };
                for (x, uv) in [
                    (-width / 4.0, [u0 + inset, v0, u1 - inset, vm]),
                    (width / 4.0, [u0 + inset, vm, u1 - inset, v1]),
                ] {
                    quad(d - k as f32 * 0.045, x, 0.0, width / 2.0, height, uv, color, 1.0);
                }
            }
        }
        // The title's panel, flying in from afar to its place.
        if t >= ARRIVE {
            let u = ((t - ARRIVE) / (END - 0.2 - ARRIVE)).clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - u).powi(3);
            quad(-24.0 * (1.0 - eased), 0.0, 0.0, size[0], size[1], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 1.0, smooth(0.0, 0.2, u)], 2.0);
        }
        // The dark over the panorama, lifting as the panel comes: the first quad, over the
        // whole image.
        let dark = 1.0 - smooth(DAWN, END - 0.6, t);
        let mut all = Vec::with_capacity(list.len() + 1);
        if dark > 0.0 {
            all.push(Quad {
                mvp: Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0)).to_cols_array(),
                uv: [0.0; 4],
                color: [0.0, 0.0, 0.0, dark],
                mode: [0.0; 4],
            });
        }
        all.extend(list);
        all.truncate(MAX_QUADS);
        queue.write_buffer(&self.quads, 0, bytemuck::cast_slice(&all));
        let atlas = atlas.create_view(&wgpu::TextureViewDescriptor::default());
        let image = image.create_view(&wgpu::TextureViewDescriptor::default());
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Horizon VR intro"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&atlas) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&image) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Horizon VR intro"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_vertex_buffer(0, self.quads.slice(..));
        pass.draw(0..4, 0..all.len() as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shader parses and validates: a mistake in it would stop the title screen in VR.
    #[test]
    fn the_intro_shader_is_valid() {
        let module = naga::front::wgsl::parse_str(SHADER).expect("parses");
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty())
            .validate(&module)
            .expect("validates");
    }

    /// The stars only ever come toward the viewer, and the rush is the fastest part.
    #[test]
    fn the_stars_rush_as_the_logo_passes() {
        let slow = travelled(1.0) - travelled(0.5);
        let rush = travelled(RUSH + 0.9) - travelled(RUSH + 0.4);
        assert!(slow > 0.0 && rush > slow * 10.0, "slow {slow}, rush {rush}");
    }
}
