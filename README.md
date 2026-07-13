# Ranami Wallpapers v0.1 beta

Ranami Wallpapers is a Windows live-wallpaper project split into two main parts:

- a C++ rendering core that attaches to the desktop `WorkerW` window and plays video through DirectX 11 / DirectComposition
- a Rust side that manages startup, tray, GUI, wallpaper library, and process-to-process communication

The project is already usable as a local beta build. The engine core is still the most low-level part, while the Rust side now handles most of the user-facing workflow.

## What works right now

- Desktop wallpaper rendering through the native C++ core
- FFmpeg hardware-decoded video playback with D3D11
- Rust launcher/client that starts and monitors the core
- Tray process
- Slint GUI for importing and browsing saved wallpapers
- Wallpaper library saved as JSON
- Async wallpaper cards with thumbnail preview, resolution, and file size
- Double-click on a wallpaper card to send that wallpaper path back to the client
- Auto-cleanup of saved wallpapers whose source file no longer exists
- Local thumbnail cache so card previews do not need to be rebuilt every time

## High-level architecture

- `RanamiWallpapers.exe`
  - Native C++ wallpaper engine core
- `rust/ranami-wallpapers`
  - Rust client/launcher
  - starts the core, manages tray lifetime, stores the current startup wallpaper, and listens for GUI commands
- `rust/ranami-wallpapers-gui`
  - Slint GUI
  - imports wallpapers, reads the JSON library, builds wallpaper cards, and sends selected paths to the client
- `rust/ranami-wallpapers-tray`
  - tray application
- `rust/shared`
  - shared constants, message IDs, JSON structs, file-name constants, and error-code definitions

More notes live in:

- [docs/rust-side-overview.md](docs/rust-side-overview.md)
- [docs/gui-wallpaper-library.md](docs/gui-wallpaper-library.md)
- [docs/how_to_use_engine_message.md](docs/how_to_use_engine_message.md)
- [docs/engine_message_list.md](docs/engine_message_list.md)

## Runtime files the app creates

- `Save-Wallpapers.json`
  - saved wallpaper library used by the GUI
- `cache-wallpaper-thumbnails/`
  - generated `.jpg` previews for wallpaper cards
- `RanamiWallpapers-startup-file.txt`
  - the last wallpaper path that the Rust client should auto-start
- `debug.txt`
  - simple error log file used by Rust-side helpers

## Wallpaper library flow

1. Importing a video in the GUI writes or updates an entry in `Save-Wallpapers.json`.
2. On refresh, the GUI first creates lightweight placeholder cards.
3. A worker thread loads extra data for each card:
   - title
   - file extension badge
   - resolution
   - file size
   - thumbnail image
4. Thumbnail images are cached in `cache-wallpaper-thumbnails/`.
5. Double-clicking a card sends only the wallpaper path to the Rust client.
6. The Rust client restarts the wallpaper core with that selected video path.

## FFmpeg and FFprobe notes

- `ffmpeg.exe` is required for thumbnail generation.
- `ffprobe.exe` is recommended for fast metadata reads.
- If `ffprobe.exe` is missing, the GUI falls back to parsing `ffmpeg -i` output for resolution.
- The GUI starts those tools with `CREATE_NO_WINDOW` so Windows does not flash a console window while thumbnails or metadata are loading.

## Requirements

- Windows 10 or 11
- Visual Studio 2022 for the C++ side
- Windows SDK
- Rust toolchain for the Rust workspace
- FFmpeg shared libraries and runtime binaries

## Build the C++ core

### Visual Studio

1. Configure FFmpeg include directories in project properties.
2. Configure FFmpeg library directories in linker properties.
3. Make sure the needed FFmpeg `.lib` files are linked.
4. Make sure the needed FFmpeg `.dll` files are placed beside the built executable.

### CMake

1. Update FFmpeg include/library paths in `CMakeLists.txt` if needed.
2. Configure the build:

```powershell
cmake -S . -B build
```

3. Build it:

```powershell
cmake --build build --config Release
```

## Build the Rust workspace

From the repo root:

```powershell
cargo build --release --manifest-path rust/Cargo.toml
```

This builds:

- `ranami-wallpapers.exe`
- `ranami-wallpapers-gui.exe`
- `ranami-wallpapers-tray.exe`

## Expected runtime files beside the launcher build

The Rust launcher currently checks for these files before startup:

- `swscale-8.dll`
- `swresample-5.dll`
- `postproc-58.dll`
- `avutil-59.dll`
- `avformat-61.dll`
- `avfilter-10.dll`
- `avdevice-61.dll`
- `avcodec-61.dll`
- `RanamiWallpapers.exe`
- `ranami-wallpapers-gui.exe`
- `ranami-wallpapers-tray.exe`
- `ffmpeg.exe`

`ffprobe.exe` is not part of the current required-file check, but the GUI will use it if it is present.

## Typical run flow

1. Start `ranami-wallpapers.exe`.
2. The Rust client checks that required files exist.
3. The client creates its hidden window and named-pipe server.
4. If `RanamiWallpapers-startup-file.txt` contains a valid path, the client starts the C++ core with that wallpaper.
5. The tray can open the GUI.
6. The GUI shows imported wallpapers from `Save-Wallpapers.json`.
7. Double-clicking a wallpaper card sends the selected path to the client.
8. The client restarts the core with the new wallpaper path and saves it as the next startup wallpaper.

## Known limits

- The project is still beta software.
- Error handling is improving, but not every failure path is fully polished yet.
- The C++ engine and the Rust launcher are connected, but the codebase still has some prototype-era structure and naming.
- Packaging and deployment are still manual.

## License

This project is licensed under the GNU Affero General Public License v3.0.

See [LICENSE](LICENSE) for the full text.

## Third-party dependencies

This project dynamically links against FFmpeg shared libraries.

The FFmpeg build used by this project may include GPL-enabled components, so check FFmpeg's own licensing terms for the exact build you ship.

See:

- [FFmpeg legal notes](https://ffmpeg.org/legal.html)
- included third-party license files

(This README was updated by Codex because the code moved ahead of the old docs.)
