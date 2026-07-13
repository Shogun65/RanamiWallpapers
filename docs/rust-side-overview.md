# Rust Side Overview

This file explains the Rust part of Ranami Wallpapers as it exists now.

## Workspace crates

### `rust/ranami-wallpapers`

This is the Rust client / launcher.

Main jobs:

- check that important runtime files exist before startup
- create the hidden client window
- start the named-pipe server
- remember the last wallpaper used for startup
- launch and monitor `RanamiWallpapers.exe`
- launch and monitor the tray process

Important files:

- `src/main.rs`
- `src/main_loop.rs`
- `src/engine.rs`
- `src/init_tray.rs`
- `src/init_gui.rs`
- `src/namepipe.rs`
- `src/window.rs`
- `src/startup_file_check.rs`

### `rust/ranami-wallpapers-gui`

This is the Slint GUI.

Main jobs:

- let the user import wallpaper videos
- save imported wallpapers into `Save-Wallpapers.json`
- build wallpaper cards from saved entries
- generate or load cached thumbnails
- read resolution and file size for each card
- send the selected wallpaper path back to the client

Important files:

- `src/main.rs`
- `src/file_saver.rs`
- `src/init_file_picker.rs`
- `src/thumbnail_cache.rs`
- `src/namepipe.rs`

### `rust/ranami-wallpapers-tray`

This is the tray app.

Main jobs:

- give the user a tray icon entry point
- send open/exit style commands back to the client window

### `rust/shared`

This crate holds values shared across the Rust side.

It currently contains:

- custom window-message IDs
- named-pipe constants
- saved-file names
- thumbnail-cache constants
- the `SaveWallpaper` struct
- the `NamePipeCommands` struct
- `EngineFailureCode`

## Startup flow

The usual startup path is:

1. `ranami-wallpapers.exe` starts.
2. `client_init()` checks that the expected runtime files exist.
3. A hidden client window is created.
4. A Tokio runtime is created.
5. The named-pipe server is started.
6. The client reads `RanamiWallpapers-startup-file.txt`.
7. If a wallpaper path exists there, the client launches `RanamiWallpapers.exe`.
8. The main loop keeps checking:
   - engine state
   - tray state
   - named-pipe commands from the GUI

## How GUI selection reaches the engine

The GUI does not talk to the C++ engine directly.

The flow is:

1. User double-clicks a wallpaper card in the GUI.
2. The GUI sends the selected wallpaper path through the named pipe.
3. The Rust client stores that path in `NamePipeCommands`.
4. The client main loop notices `wallpaper_changed == true`.
5. The current core process is stopped.
6. The client starts a new `RanamiWallpapers.exe` process with the new video path.
7. The new path is also saved to `RanamiWallpapers-startup-file.txt`.

## Client restart behavior

`main_loop.rs` treats a non-zero engine exit as a crash and may try to restart the core with the last wallpaper.

Current protections:

- it stops retrying after too many crashes
- it can also stop when the shared hard-crash flag is set

This is one of the areas that will likely keep evolving as the engine-side error handling gets stronger.

## Files the Rust side writes

- `Save-Wallpapers.json`
  - wallpaper library for the GUI
- `cache-wallpaper-thumbnails/`
  - cached card preview images
- `RanamiWallpapers-startup-file.txt`
  - last wallpaper used by the launcher
- `debug.txt`
  - simple text error log

## FFmpeg tool usage on the Rust side

The GUI uses helper binaries for two separate jobs:

- `ffmpeg.exe`
  - generate one preview frame for a wallpaper card
- `ffprobe.exe`
  - read resolution cleanly when available

If `ffprobe.exe` is missing, the GUI falls back to parsing `ffmpeg -i` output.

Because those tools are console applications, the GUI launches them with the Windows `CREATE_NO_WINDOW` flag so thumbnail generation does not flash a CMD window.

## Good places to read first

If you are new to the Rust side, start here:

1. `rust/shared/src/lib.rs`
2. `rust/ranami-wallpapers/src/main.rs`
3. `rust/ranami-wallpapers/src/main_loop.rs`
4. `rust/ranami-wallpapers-gui/src/main.rs`
5. `rust/ranami-wallpapers-gui/src/thumbnail_cache.rs`
