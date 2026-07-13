# GUI Wallpaper Library Flow

This doc explains how the Slint GUI builds and refreshes the wallpaper library.

## Main storage files

- `Save-Wallpapers.json`
  - the saved wallpaper list
- `cache-wallpaper-thumbnails/`
  - cached `.jpg` previews generated from videos

## Saved wallpaper JSON format

Each saved wallpaper is currently stored as:

```json
{
  "name": "furina-masquerade",
  "path": "C:\\Users\\name\\Downloads\\furina-masquerade.mp4"
}
```

The struct comes from `rust/shared/src/lib.rs`:

```rust
pub struct SaveWallpaper {
    pub name: String,
    pub path: String,
}
```

## Import flow

The import path is handled in `rust/ranami-wallpapers-gui/src/file_saver.rs`.

Flow:

1. The user chooses a video in the async file picker.
2. The GUI builds a `SaveWallpaper` entry from the file name and full path.
3. `Save-Wallpapers.json` is read.
4. If that exact path already exists, the entry is updated instead of duplicated.
5. Otherwise a new entry is appended.
6. The JSON file is written back with pretty formatting.

## Missing-file cleanup

The GUI does not keep dead wallpaper entries forever.

When the library is refreshed, `read_existing_saved_wallpapers()` removes entries whose `path` no longer exists on disk and writes the cleaned JSON back.

That means:

- renaming a video outside the app will break the old saved path
- the broken entry will disappear on the next refresh / relaunch
- its thumbnail cache can then be cleaned too

## Card refresh flow

The refresh flow lives in `rust/ranami-wallpapers-gui/src/main.rs`.

The GUI does not block the window while loading wallpaper info.

Flow:

1. Read the saved wallpapers.
2. Build placeholder cards immediately.
3. Start a worker thread.
4. On that worker thread, for each wallpaper:
   - compute display title
   - compute format badge
   - compute detail text
   - load or generate the thumbnail
5. Send the finished card data back to the UI thread.
6. Replace the placeholder row with the resolved card.

## Why there is a generation counter

The refresh generation counter prevents old worker threads from writing stale data into the UI after a newer refresh has already started.

That matters when:

- the user imports another wallpaper quickly
- the library is refreshed again before the previous background load finishes

## Wallpaper details shown on cards

The detail line currently tries to show:

- resolution
- file size

Example:

```text
3840x2160 | 92.4 MB
```

Resolution logic:

1. try `ffprobe.exe`
2. if that fails, parse `ffmpeg -i` output

File size logic:

- read file metadata directly from the filesystem

## Thumbnail generation

Thumbnail generation lives in `rust/ranami-wallpapers-gui/src/thumbnail_cache.rs`.

Current behavior:

- generate one frame around `00:00:01`
- resize/crop to `640x360`
- save as `.jpg`
- store it in `cache-wallpaper-thumbnails/`

The cache file name is built from:

- a sanitized video stem
- a stable hash of the full original path

This avoids collisions when two different folders contain files with the same name.

## When a thumbnail is rebuilt

A thumbnail is regenerated when:

- the cached file does not exist
- the source video is newer than the cached thumbnail

## Stale thumbnail cleanup

The GUI also removes cached thumbnails that no longer belong to any saved wallpaper entry.

This cleanup runs during library refresh so the cache folder does not keep growing forever with dead previews.

## Why thumbnail loading does not freeze the GUI

The expensive work happens on a normal worker thread, not on the Slint UI thread.

That expensive work includes:

- file metadata reads
- `ffprobe.exe`
- `ffmpeg.exe`
- image decoding from disk

The UI thread only receives the finished card data and swaps it into the model.

## Why CMD windows should not flash anymore

`ffmpeg.exe` and `ffprobe.exe` are console applications.

Even if the GUI binary uses:

```rust
#![windows_subsystem = "windows"]
```

Windows can still flash a console window for child console processes.

To avoid that, the GUI starts those tools with the Windows `CREATE_NO_WINDOW` flag.

## Card click behavior

Wallpaper cards now use double-click behavior for selection.

On double-click:

- the GUI sends only the saved wallpaper path
- it does not do extra path validation in the callback itself
- path cleanup is handled during refresh/relaunch time instead
