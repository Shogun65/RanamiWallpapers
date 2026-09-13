# 🌸 Ranami Wallpapers

**A lightweight live wallpaper application for Windows.**

Ranami Wallpapers lets you use video files as animated desktop wallpapers while keeping the playback engine focused on efficient GPU-based video decoding and rendering.

> **Status:** Beta — v0.3.0

---

## ✨ Features

- 🎥 Video live wallpapers
- 🖼️ Wallpaper library with thumbnail previews
- 📂 Import and save wallpaper videos
- 📊 Wallpaper information such as resolution and file size
- 🖱️ Double-click a wallpaper to apply it
- 🧩 System-tray controls
- 🚀 Start with Windows
- 💾 Automatically remembers the last wallpaper
- 🗂️ Local wallpaper library and thumbnail cache
- ⏸️ Reduces rendering work when a maximized window is detected

---

## 🖥️ How Ranami Works

Ranami is split into several processes, each handling a specific part of the application.

```text
                    ┌─────────────────────┐
                    │     Slint GUI       │
                    │                     │
                    │ Library / Import /  │
                    │ Wallpaper Selection │
                    └──────────┬──────────┘
                               │
                         Named Pipe IPC
                               │
                               ▼
                    ┌─────────────────────┐
                    │   Rust Client       │
                    │                     │
                    │ Process Manager     │
                    │ Startup / Commands  │
                    │ Engine Monitoring   │
                    └──────────┬──────────┘
                               │
                         Process Launch
                               │
                               ▼
                    ┌─────────────────────┐
                    │   C++ Engine        │
                    │                     │
                    │ FFmpeg + D3D11VA    │
                    │ Frame Queue         │
                    │ Video Processor     │
                    │ DirectComposition   │
                    └──────────┬──────────┘
                               │
                               ▼
                         Windows Desktop
```

The GUI does **not** communicate directly with the native playback engine. The Rust client sits between them and manages the engine process and wallpaper state.

---

## ⚙️ Playback Pipeline

The native engine uses FFmpeg together with Direct3D 11 hardware acceleration.

```text
Video File
    │
    ▼
FFmpeg Demuxer
    │
    ▼
Hardware Decoder (D3D11VA)
    │
    ▼
GPU NV12 Decode Surfaces
    │
    ▼
Bounded Frame Queue
    │
    ▼
PTS-Based Playback Timing
    │
    ▼
D3D11 Video Processor
    │
    ├── Scaling
    ├── NV12 / YCbCr → RGB
    └── Colour-space handling
    │
    ▼
DirectComposition Swap Chain
    │
    ▼
Windows Desktop
```

Decoded video frames remain in GPU memory on the normal hardware-decoding path instead of being copied through the CPU for every frame. The D3D11 video processor performs the final scaling and colour conversion before presentation.

The playback engine also uses a bounded frame queue and a separate decoder thread so decoding and rendering can operate independently.

---

## 🧱 Project Structure

```text
RanamiWallpapers/
│
├── DComp/                  # DirectComposition integration
├── DX/                     # Direct3D 11 device setup
├── DXVA/                   # D3D11 video processing
├── FFmpeg/                 # FFmpeg initialization and frame pipeline
├── Parse/                  # Engine command-line parsing
├── PostMessageW/           # Windows message communication
├── Render/                 # Playback timing and rendering logic
├── SwapChin/               # Direct3D swap chain
├── Window/                 # Win32 window + WorkerW integration
│
├── rust/
│   ├── ranami-wallpapers/          # Main Rust client / launcher
│   ├── ranami-wallpapers-gui/      # Slint wallpaper-library GUI
│   ├── ranami-wallpapers-tray/     # System tray application
│   └── shared/                     # Shared IPC and application types
│
├── docs/                   # Architecture and developer documentation
├── installer/              # Windows MSI installer
│
├── CMakeLists.txt
├── Engine.cpp
├── Engine.h
├── main.cpp
└── LICENSE
```

The Rust side is organized as a Cargo workspace containing the client, GUI, tray application, and shared crate.

---

## 🖼️ Wallpaper Library

The GUI maintains a local wallpaper library.

Imported wallpapers are stored as references to their original files rather than being copied into the application directory.

The library stores entries similar to:

```json
{
  "name": "furina-masquerade",
  "path": "C:\\Videos\\furina-masquerade.mp4"
}
```

Ranami also maintains a thumbnail cache generated with FFmpeg.

Thumbnails are:

- generated around the first second of the video
- resized/cropped to `640x360`
- stored as JPEG
- regenerated when the source video is newer
- cleaned up when they are no longer referenced

Wallpaper information is loaded in a worker thread so expensive FFmpeg and filesystem operations do not block the GUI.

---

## 🎞️ Video Compatibility

Ranami relies on the **codec inside the video**, not simply the filename extension.

The engine currently uses a hardware-only D3D11 decoding path, so a video must have a codec and hardware/driver combination supported by the available FFmpeg D3D11VA path.

Good choices are generally:

- MP4 / MKV with H.264
- MP4 / MKV with H.265 / HEVC
- Other codecs supported by your GPU's hardware decoder

Container formats such as AVI, ASF, MKV, or OGG should **not** be assumed to work universally because the actual video codec determines hardware compatibility.

---

## 📁 Application Data

Ranami stores application data under:

```text
%LOCALAPPDATA%\RanamiWallpapers
```

This includes things such as:

```text
Save-Wallpapers.json
cache-wallpaper-thumbnails/
RanamiWallpapers-startup-file.txt
debug.txt
```

These files are used for the wallpaper library, cached previews, startup restoration, and basic error logging.

---

## 📦 Installation

The easiest way to install Ranami is through the latest release.

1. Download the latest MSI from **Releases**
2. Run the installer
3. Launch **Ranami Wallpapers**
4. Import your wallpaper videos
5. Double-click a wallpaper to apply it

### Latest Release

**v0.3.0 Beta**

The current release includes:

- Wallpaper library
- Thumbnail generation
- Video information
- System tray integration
- Startup handling
- MSI installation
- Improved wallpaper switching and engine process management
- Reduced rendering while maximized windows are active

---

## 🛠️ Building From Source

### Requirements

- Windows 10 or Windows 11
- Visual Studio / MSVC
- CMake
- Rust toolchain
- FFmpeg development libraries
- FFmpeg command-line tools

The native engine is built as a C++17 Windows executable and links against FFmpeg, Direct3D 11, DXGI, DirectComposition, and Windows COM libraries.

The Rust applications are built through the workspace in:

```text
rust/
```

Build the Rust workspace with:

```powershell
cd rust
cargo build --release
```

The native engine is built with CMake.

> **Note:** The current `CMakeLists.txt` contains a local FFmpeg include/library path, so those paths need to be adjusted for another development machine before building the C++ engine.

---

## 📚 Developer Documentation

More detailed documentation is available in [`docs/`](docs/).

Useful starting points:

- [`architecture.md`](docs/architecture.md) — complete playback architecture
- [`rust-side-overview.md`](docs/rust-side-overview.md) — Rust applications and IPC
- [`gui-wallpaper-library.md`](docs/gui-wallpaper-library.md) — wallpaper library and thumbnail system
- [`engine_message_list.md`](docs/engine_message_list.md) — engine message definitions
- [`how_to_use_engine_message.md`](docs/how_to_use_engine_message.md) — native message communication

---

## 🚧 Current Limitations

Ranami is still in beta, so some rough edges remain.

Current areas under development include:

- broader hardware/codec compatibility
- improved error reporting
- playback loop correctness
- power/resource optimisation
- additional wallpaper features
- further GUI development

Also note that **4K video still requires 4K decoding work even on a 1080p monitor**. The final image is scaled down before presentation, but the original source still has to be decoded first.

---

## 🎯 Project Goal

Ranami started as an experiment in building a lightweight Windows live-wallpaper system from scratch.

The project combines:

- **C++** for the native playback engine
- **Direct3D 11 / D3D11VA** for GPU video processing
- **FFmpeg** for video decoding and tooling
- **Rust** for application management and IPC
- **Slint** for the graphical interface
- **Win32 / DirectComposition** for integrating the wallpaper with the Windows desktop

The goal is to keep the application simple, efficient, and fully under the project's control.

---

## 📄 License

Ranami Wallpapers is licensed under the **GNU Affero General Public License v3.0**.

See [`LICENSE`](LICENSE) for the complete license text.

Ranami Wallpapers uses **FFmpeg**. See the FFmpeg legal information for its licensing terms.

---

## ⭐ Project

**Ranami Wallpapers**  
A Windows live wallpaper project built from the ground up.

[GitHub Repository](https://github.com/Shogun65/RanamiWallpapers)
