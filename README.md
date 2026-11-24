<p align="center">
  <img src="docs/media/icon-256.png" width="128" alt="HorizonCraft VR">
</p>

<h1 align="center">HorizonCraft VR</h1>

<p align="center">
  Standalone VR for <b>Meta Quest 3, 3S and 2</b>, on Horizon: Minecraft: Java Edition 26.3 rewritten in Rust
</p>

<p align="center">
  <a href="https://github.com/Boboetto/HorizonCraft-VR/releases/latest"><img src="https://img.shields.io/github/v/release/Boboetto/HorizonCraft-VR?label=release" alt="Latest release"></a>
  <img src="https://img.shields.io/badge/Meta%20Quest-3%20%7C%203S%20%7C%202-1C1E20" alt="Meta Quest 3, 3S and 2">
  <img src="https://img.shields.io/badge/Minecraft-Java%2026.3-5D9C3F" alt="Minecraft Java 26.3">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License MIT"></a>
</p>

> **Unofficial fan project.** Not affiliated with, endorsed by or connected to Mojang, Microsoft or Meta.
> You need to **own Minecraft: Java Edition**. This repository and its releases contain **no Mojang files**: the app
> downloads them from Mojang on the headset.

HorizonCraft VR is a standalone APK for the **Meta Quest 3 and 3S**, and it runs on the **Quest 2** too (thanks to the players who tested it there; lower the resolution and chunks for a smooth game): no PC, no streaming.
It plays Minecraft: Java Edition 26.3 in VR, with motion controls in the spirit of Vivecraft and QuestCraft.

**1.0** is the first stable release, tested on the Quest 3: a steady 72 Hz with Application SpaceWarp, terrain drawn
in a few big batches, MSAA 4x that also smooths water, ice, glass, grass and leaves, gestures on Vivecraft's rules,
a new launcher with remappable controls and snap turning, and in-app updates. 0.3.x installs update in place (worlds stay).

### New in 1.0

- **New launcher**: a sharp, modern look drawn at the panel's full resolution, a side menu (Home, Skin, Settings,
  Controls, Mods & packs, Account, Updates) and an update badge on the home page
- **Controls page**: give any button's action to another button, swap the sticks (left-handed play), smooth or snap
  turning (30°, 45°, 90°)
- **Mods & packs page**: install mods and see or remove what is in the mods, resource pack and shader pack folders
- Swim and glide with the elytra in every direction, where your head points
- The bow in the right hand, drawn with the left; the arrow flies where the bow points
- The trident keeps its look while you charge a throw; boats turn smoothly, also on ice
- Smaller arms and held items; hands no longer pushed back from blocks, so you can dig close up

## Features

- Stereo VR with head tracking; both arms follow the Touch controllers, items in the hands
- Hotbar on the left forearm: hover it with the right controller and pull the trigger to pick a slot
- A crosshair for each hand, lying on the face of the block it aims at
- The left trigger uses and places along the left hand while the right trigger attacks along the right one
- Gestures modelled on Vivecraft's (LGPLv3) rules: swing a weapon or tool at a mob, strike blocks to mine them (each blow
  digs a little, the crack waits for the next one), bring food to the mouth to eat or tip a drink, draw the stroke
  of a swim towards you, nock and draw a bow with both hands, move the arms to row a boat; and our own, measured the same
  way: throw potions, snowballs, eggs and ender pearls with a stroke of the hand, raise a trident beside the head and
  throw it, flick a fishing rod out and back, hold a shield up in front, cock a crossbow with the other hand
- Menus on a panel in the room with a pointer for each hand, 360° title panorama
- Walk where you look with the left stick, turn with the right one (smooth or snap; sticks and buttons remappable in the launcher)
- A Quality setup tuned for the Quest 3 (1.3x resolution, MSAA 4x antialiasing, 72 Hz with SpaceWarp, 6 chunks, anisotropic filtering), with a resolution step and a performance boost in the launcher
- A tool strikes with its tip
- Chat on the panel in the room, typed with the Quest's own keyboard
- In-app updates: the launcher shows an **Update** button when a new release is out

## Controls

The default controls; the launcher's **Controls** page can give the buttons' actions to other buttons, swap the
sticks and turn on snap turning.

| Input | In play | In a menu |
|---|---|---|
| Left stick | walk (where you look) | – |
| Right stick | turn | scroll |
| Right trigger | attack / mine / pick a wrist slot | click with the right pointer |
| Left trigger | use / place along the left hand; with a bow in the right hand: draw the string | click with the left pointer |
| A | jump | confirm |
| B | chat (B again closes it) | back; in an inventory or container: quick move (craft all, send the stack to the other side) |
| Y | inventory | close the inventory |
| X | drop one item | drop one item (carried stack or slot) |
| X held + Y | drop the rest of the stack | drop the rest of the stack |
| Left grip | swap the item with the off hand | – |
| Both grips | debug screen (F3) | – |
| Right grip | – | place one item of the carried stack; with the trigger: quick move |
| Left grip (in a container) | – | take half of the stack |
| Both grips (in a container) | – | move every stack of that item to the other side |
| Left / right stick click | sprint / sneak | – |
| Menu button | pause | – |

## Install on the Meta Quest 3, 3S or 2

1. Turn on **developer mode** for your Quest: in the **Meta Horizon** app on your phone → Menu → Devices →
   your headset → Headset settings → Developer mode (Meta asks you to create a free developer account once).
2. Download **`HorizonCraft-VR-v*.apk`** from the [latest release](https://github.com/Boboetto/HorizonCraft-VR/releases/latest).
3. Install it with **SideQuest**: connect the headset to the computer with its USB-C cable, accept "Allow USB debugging"
   in the headset, then drag the APK onto the SideQuest window.
   (Or drag it onto **Meta Quest Developer Hub**.)
4. In the headset, open **App Library → Unknown sources → HorizonCraft VR**.

## First start

Press **Download game files** and wait a few minutes, on Wi-Fi:

1. the headset downloads the official Minecraft: Java Edition 26.3 files from Mojang (about 700 MB);
2. HorizonCraft VR prepares its game data on the headset itself, with a Java runtime built into the app (about a minute);
3. the game starts. Next time it opens straight away.

Later updates: tap **Update** in the launcher's *Updates* section and confirm the install (allow installs from HorizonCraft VR the first time). Your worlds stay.

No computer needed after installing. You need to own Minecraft: Java Edition.

## In italiano

1. **Installa:** attiva la modalità sviluppatore (app Meta Horizon sul telefono → Dispositivi → visore →
   Impostazioni → Modalità sviluppatore) e installa l'APK dell'ultima release trascinandolo su **SideQuest**.
2. **Apri** HorizonCraft VR da **Libreria → Origini sconosciute**.
3. **Premi "Download game files"** e aspetta qualche minuto col Wi-Fi: il visore scarica i file ufficiali da Mojang,
   prepara da solo i dati di gioco e avvia HorizonCraft VR. Nessun PC necessario. Serve possedere Minecraft: Java Edition.

## Building

```sh
OPENXR_LOADER=/path/to/libopenxr_loader.so ANDROID_HOME=~/android/sdk ./packaging/android/build_apk.sh
```

`libopenxr_loader.so` is the Khronos OpenXR loader for Android (`org.khronos.openxr:openxr_loader_for_android`,
Apache 2.0). For the first start to prepare the game data on the headset, set `HORIZON_APK_ASSETS` to a folder with
`horizon-jre/` (a Java 25 runtime for Android arm64, plus the NDK's `libc++_shared.so` in its `lib/`) and
`horizon-prepare/` (`research-lock.json`, `mods/horizon/src/test/java/dev/horizonlab/parity/` and the compiled
`exporters/` from `horizon-prepare compile-exporters`). The VR code is in `crates/horizon-xr`, the renderer's `render_xr` and
`crates/horizon-client/src/game/android_input.rs`.

## Credits

- **Horizon**, Minecraft: Java Edition rewritten in Rust, by its authors (MIT)
- **Horizon for Android** by [Rayan-XOO](https://github.com/Rayan-XOO/horizon) (MIT), which this project forks
- The Khronos OpenXR loader (Apache 2.0)
- The Java 25 runtime for Android from [AngelAuraMC/angelauramc-openjdk-build](https://github.com/AngelAuraMC/angelauramc-openjdk-build)
  (OpenJDK, GPLv2 with the Classpath Exception; source: [openjdk/jdk](https://github.com/openjdk/jdk)), its licences in the APK's `assets/horizon-jre/legal`
- Gestures modelled on the rules of [Vivecraft](https://github.com/Vivecraft/VivecraftMod) (LGPLv3); inspired by Vivecraft and QuestCraft

Minecraft is a trademark of Mojang Synergies AB. Meta Quest is a trademark of Meta Platforms, Inc.
