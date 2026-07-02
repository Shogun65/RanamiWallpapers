# this peoject is ended because of bullshit UI i hate making UI

# Wallpaper Engine V0.08

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
- First-run video picker using the Windows file dialog
- Last selected wallpaper path is saved to `wallpaper_path.txt` and auto-loaded on the next launch

New in v0.08:
- Startup flow no longer depends on `-f- <video_path>` command-line arguments
- Better startup usability: if no saved path exists, the app asks you to choose a video file
- README run/build notes updated to match the current executable flow

Not implemented yet (as of v0.08):
- In-app UI/settings
- Dynamic wallpaper selection
- Audio playback
- Full process lifecycle polish (startup/service UX, tray controls, etc.)

## Tech Stack

- C++17
- Win32 API
- Direct3D 11 (`d3d11`)
- DXGI (`dxgi`)
- DirectComposition (`dcomp`)
- FFmpeg shared build (`avcodec`, `avformat`, `avutil`, `swscale`)

## Requirements

- Windows 10/11
- Visual Studio 2022 (MSVC toolset `v145`)
- Windows SDK (`10.0` target in project)
- FFmpeg shared build with headers/libs/dlls available locally
- CMake 3.10+ if you want to build with the included `CMakeLists.txt`

## Project Layout

- `main.cpp`: app entry, saved-path loading, and first-run file picker flow
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
4. Make FFmpeg runtime DLLs available to the executable:
   - Copy FFmpeg `bin/*.dll` next to the built `.exe`, or
   - Add FFmpeg `bin` directory to your system/user `PATH`.

## Build Setup (CMake)

1. Update the FFmpeg include/library paths inside `CMakeLists.txt` if your FFmpeg folder is in a different location.
2. Configure the project:
   - `cmake -S . -B build`
3. Build the project:
   - `cmake --build build --config Release`
4. The generated executable is:
   - `build\Release\AliveWallpaperEngine.exe`

## Run

1. Build and run from Visual Studio or CMake.
2. On first launch, the app opens a file picker so you can choose a wallpaper video.
3. After you choose a file once, the path is saved to `wallpaper_path.txt`.
4. On later launches, the app loads that saved path automatically and starts rendering without extra command-line arguments.
5. The app attaches to desktop `WorkerW` and starts rendering as wallpaper.

## Notes

- Queue size is currently passed from `main.cpp`:
  - `engine.MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(..., buffersize);`
  - Current default: `int buffersize = 3;`
- Frame queue clamps buffer size to `[3, 18]`.
- Current startup flow:
  - Try loading the video path from `wallpaper_path.txt`
  - If the file is missing or empty, open the Windows file picker
  - Save the selected path for the next run
- The app sets `PER_MONITOR_AWARE_V2` DPI awareness on startup.
- Debug console logging is currently disabled in the default startup flow.

## Troubleshooting

- FFmpeg include/lib path errors:
  - Re-check project property paths for the active configuration (`x64 Debug` vs `x64 Release`).
- Missing DLL errors on launch:
  - Ensure FFmpeg runtime DLLs are in the executable directory or `PATH`.
- No video appears on first launch:
  - Make sure you selected a supported video file in the picker.
- Wrong video keeps loading:
  - Delete `wallpaper_path.txt` and launch again to pick a different file.
- Black/green output:
  - Validate that decode is running and frames are being pushed/popped (watch debug console).
- WorkerW attach failure:
  - The app currently exits with an error dialog if suitable `WorkerW` is not found.

## License

This project is licensed under the GNU Affero General Public License v3.0.

See [LICENSE](LICENSE) for full text.
(This README update was written by Codex because iam lazy)
(and also iam not good whit license stuff but i try to understand as much iam)

## Third-Party Dependencies

This project dynamically links against FFmpeg shared libraries.

The distributed FFmpeg build used by this project is GPL-enabled
(`--enable-gpl`) and licensed under the GNU General Public License
subject to FFmpeg's own licensing terms.

See:
- https://ffmpeg.org/legal.html
- Included third-party license files
