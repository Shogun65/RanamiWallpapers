# Wallpaper Engine V0.07

A lightweight Windows live-wallpaper prototype written in C++ with DirectX 11, DirectComposition, and FFmpeg hardware decode (D3D11VA).

This app creates a borderless composition window, attaches it to the desktop `WorkerW`, decodes video with FFmpeg on the GPU, and renders frames to the desktop background.

## Current Status

Early-stage prototype focused on performance and core rendering pipeline.

Implemented:
- `WorkerW` desktop attachment
- DirectX 11 device + composition swap chain
- DirectComposition visual pipeline
- FFmpeg video decode with D3D11 hardware frames
- Frame queue/pool with a dedicated decoder thread
- Basic PTS-based playback timing and loop-at-EOF behavior

Not implemented yet (as of v0.07):
- In-app UI/settings
- Dynamic wallpaper selection
- Audio playback
- Full process lifecycle polish (startup/service UX, tray controls, etc.)

## Tech Stack

- C++20
- Win32 API
- Direct3D 11 (`d3d11`)
- DXGI (`dxgi`)
- DirectComposition (`dcomp`)
- FFmpeg shared build (`avcodec`, `avformat`, `avutil`, `swscale`, `swresample`, `avdevice`)

## Requirements

- Windows 10/11
- Visual Studio 2022 (MSVC toolset `v145`)
- Windows SDK (`10.0` target in project)
- FFmpeg shared build with headers/libs/dlls available locally

## Project Layout

- `main.cpp`: app entry + command-line parsing (`-f- <video_path>`)
- `Engine.*`: high-level app orchestration
- `WorkerW.*`: desktop `WorkerW` discovery/attachment logic
- `Window.*`: wallpaper window creation + message loop
- `DXDevice.*`, `SwapChain.*`, `DComp.*`: rendering/composition setup
- `FFmpeg.*`, `DecoderLoop.inl`: decode pipeline + hardware context setup
- `FrameQueue.cpp`, `FramePool.cpp`: producer/consumer buffering
- `DXVA.*`: video processor path from decoded NV12 frames to backbuffer
- `Render.*`: frame presentation and playback timing

## Build Setup (Visual Studio)

1. Configure FFmpeg include path:
   - Project Properties -> `C/C++` -> `General` -> `Additional Include Directories`
2. Configure FFmpeg library path:
   - Project Properties -> `Linker` -> `General` -> `Additional Library Directories`
3. Ensure these linker inputs are present:
   - `avcodec.lib`
   - `avformat.lib`
   - `avutil.lib`
   - `swscale.lib`
   - `swresample.lib`
   - `avdevice.lib`
4. Make FFmpeg runtime DLLs available to the executable:
   - Copy FFmpeg `bin/*.dll` next to the built `.exe`, or
   - Add FFmpeg `bin` directory to your system/user `PATH`.

## Run

1. Build and run from Visual Studio (x64 Debug/Release).
2. Pass the wallpaper video path through command arguments using this format:
   - `-f- C:\path\to\your\video.mp4`
3. Example:
   - `.\x64\Release\Wallpaper_Engine_V0.06.exe -f- "C:\Videos\wallpaper.mp4"`
4. The app attaches to desktop `WorkerW` and starts rendering as wallpaper.

## Notes

- Queue size is currently passed from `main.cpp`:
  - `engine.MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(..., buffersize);`
  - Current default: `int buffersize = 3;`
- Frame queue clamps buffer size to `[3, 18]`.
- Debug console logging is enabled in the current flow.

## Troubleshooting

- FFmpeg include/lib path errors:
  - Re-check project property paths for the active configuration (`x64 Debug` vs `x64 Release`).
- Missing DLL errors on launch:
  - Ensure FFmpeg runtime DLLs are in the executable directory or `PATH`.
- Black/green output:
  - Validate that decode is running and frames are being pushed/popped (watch debug console).
- WorkerW attach failure:
  - The app currently exits with an error dialog if suitable `WorkerW` is not found.

## License

This project is licensed under the GNU Affero General Public License v3.0.

See [LICENSE](LICENSE) for full text.
(This Writed by Codex Because iam lazy)
(and also iam not good whit license stuff but i try to understand as much iam)

## Third-Party Dependencies

This project dynamically links against FFmpeg shared libraries.

The distributed FFmpeg build used by this project is GPL-enabled
(`--enable-gpl`) and licensed under the GNU General Public License
subject to FFmpeg's own licensing terms.

See:
- https://ffmpeg.org/legal.html
- Included third-party license files
