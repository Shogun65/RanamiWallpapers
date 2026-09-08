# Ranami Playback Architecture

This page describes how the project works **as the code is written today**. It
covers the Rust launcher, GUI and tray applications, the native C++ playback
engine, FFmpeg, Direct3D 11, and the hand-off between them.

It is a technical map of the current implementation, not a promise that every
feature or video format is supported.

## The programs involved

Ranami is made of separate processes, each with a focused job:

| Program | Language | Responsibility |
| --- | --- | --- |
| `ranami-wallpapers.exe` | Rust | Main client/launcher. Owns process lifecycle, the hidden client window, the named-pipe server, startup restoration, and the tray process. |
| `ranami-wallpapers-gui.exe` | Rust + Slint | Wallpaper-library window. Imports files, saves the library, creates thumbnails, and tells the client which wallpaper to use. |
| `ranami-wallpapers-tray.exe` | Rust | System-tray menu for opening the GUI and exiting Ranami. |
| `RanamiWallpapers.exe` | C++ | Native live-wallpaper engine. Decodes, scales, colour-converts, and presents video behind the desktop icons. |
| `ffmpeg.exe` / `ffprobe.exe` | FFmpeg tools | Used by the Rust side for thumbnail generation and video metadata. The C++ engine instead links directly against the FFmpeg libraries. |

The `shared` Rust crate contains the common named-pipe structure, message IDs,
storage paths, and error-code definitions used by the Rust programs.

## Full wallpaper-selection flow

```text
User double-clicks a wallpaper card
                 |
                 v
Rust GUI serializes { video_path, wallpaper_changed: true }
                 |
                 | Windows named pipe: \\.\pipe\RanamiWallpapers
                 v
Rust client updates its shared command state
                 |
                 v
Rust client stops the old C++ engine, if any
                 |
                 +--> creates/sets a cached static JPEG wallpaper in parallel
                 |
                 v
Rust client launches RanamiWallpapers.exe with three arguments
  1. video path
  2. frame-queue size (currently "3")
  3. hidden Rust client-window HWND
                 |
                 v
C++ engine creates the live wallpaper behind desktop icons
```

The GUI does **not** send the selected path directly to the C++ engine. The
Rust client is the process manager between them.

## Rust side in detail

### 1. Client startup

The Rust client starts by checking that its runtime files are present. This
includes the C++ engine, the GUI and tray executables, `ffmpeg.exe`, and the
FFmpeg DLLs. It then:

1. Creates a hidden Win32 client window.
2. Starts a Tokio runtime.
3. Starts a Windows named-pipe server.
4. Reads the last wallpaper path from LocalAppData.
5. Starts the C++ engine if that path still exists.
6. Enters the main management loop.

The client creates a global mutex. If another client is already running, the
new process opens the GUI instead of starting a second client.

### 2. GUI import and library

The Slint GUI's file picker normally shows these extensions:

```text
mp4, mkv, avi, mov
```

It also offers an **all files** filter. The picker filter is only a UI
convenience; it does not prove that a video will be playable by the C++ engine.

When a file is imported, the GUI stores its file name and full path in
`Save-Wallpapers.json` under:

```text
%LOCALAPPDATA%\RanamiWallpapers
```

The original video is not copied or converted during import. Ranami keeps a
reference to its original path, so moving or deleting that video makes the
saved library entry invalid and it is removed during a later library refresh.

The GUI creates a thumbnail cache under the same LocalAppData folder. It uses
the external `ffmpeg.exe` to extract one JPEG preview frame and uses
`ffprobe.exe` when available to read video dimensions. If `ffprobe.exe` is not
available, it falls back to parsing `ffmpeg -i` output.

### 3. GUI to client communication

On a card double-click, the GUI sends JSON such as:

```json
{"video_path":"C:\\Videos\\wallpaper.mkv","wallpaper_changed":true}
```

to `\\.\pipe\RanamiWallpapers`.

The client named-pipe server reads that JSON and replaces its shared
`NamePipeCommands` state. The client main loop checks that state roughly every
10 ms. Once it sees `wallpaper_changed`, it starts the switch process.

### 4. Wallpaper switching and process management

For a new selection, the Rust client:

1. Stops and waits for the existing C++ engine process.
2. Clears the saved native-engine HWND and stops the tray child for that
   engine session.
3. Starts generation of a static JPEG wallpaper on a background thread.
4. Starts a new C++ engine process with the selected video path.
5. Saves the selected path as the startup wallpaper.
6. Starts the tray application again when needed.

The static JPEG is a fallback background. It is generated at the primary
screen's dimensions with FFmpeg using a scale-and-crop filter, then set through
`SystemParametersInfoW(SPI_SETDESKWALLPAPER)`. The live C++ window is still the
thing that displays the moving video.

If the C++ engine exits with a non-zero status, the client treats it as a crash
and may retry the current wallpaper. It stops retrying after more than five
crashes or after the shared hard-crash flag is set.

### 5. Windows-message communication

The Rust client has a hidden window to receive messages from the tray and C++
engine. The most important currently-active native hand-off is:

```text
C++ engine window created
        |
        | WM_ENGINE_SENT_HWND, lParam = engine HWND
        v
Hidden Rust client window stores ENGINE_HWND
```

The tray sends `WM_ENGINE_OPEN_GUI` or `WM_ENGINE_EXIT` to the hidden client
window. On exit, the client forwards the exit message to the stored native
engine HWND.

The shared crate also defines startup-failure message IDs, but the present C++
code mainly uses message boxes and `std::exit`; it does not yet report detailed
failure codes back to the Rust client.

## C++ engine startup

The native executable receives the three arguments sent by the Rust client:

```text
RanamiWallpapers.exe <video path> <buffer count> <client HWND>
```

`Parse::get_data()` converts the UTF-16 command-line path to UTF-8, parses the
buffer count, and converts the client HWND string back into a Windows `HWND`.

The engine then performs this setup order:

```text
parse arguments
    -> create frame queue and AVFrame wrapper pool
    -> create a popup wallpaper window
    -> find WorkerW and parent the popup beneath desktop icons
    -> create the D3D11 device and immediate context
    -> initialise FFmpeg with D3D11VA hardware decode
    -> start the decoder thread
    -> create a DirectComposition swap chain and its back buffer
    -> initialise the D3D11 video processor
    -> show the window and send its HWND to the Rust client
    -> enter the render/message loop
```

### WorkerW and DirectComposition

The engine asks Explorer's `Progman` window to create/find a `WorkerW` window.
It parents its own borderless popup window to the empty WorkerW. That is what
places the video behind the desktop icons.

The D3D11 swap chain is created for DirectComposition, and a DirectComposition
visual uses that swap chain as its content. The visible surface is therefore a
composition swap chain rather than a normal application window swap chain.

## C++ video pipeline

```text
Video file
   |
   v
FFmpeg demuxer: avformat_open_input / av_read_frame
   |
   | compressed video packets only
   v
FFmpeg decoder using D3D11VA
   |
   | native-resolution NV12 D3D11 texture surfaces
   v
bounded frame queue
   |
   v
render loop, timed by video PTS
   |
   v
D3D11 video processor: scale + YCbCr/NV12 to RGB conversion
   |
   v
monitor/window-sized BGRA swap-chain back buffer
   |
   v
DirectComposition -> Windows desktop compositor -> monitor
```

### 1. Opening the input

`avformat_open_input()` asks FFmpeg to open and probe the supplied file. The
engine then finds the first stream marked as video and asks FFmpeg for a
decoder for that stream's codec.

The extension does not control this step. For example, an `.asf` file can hold
several different codecs, and an `.mkv` file can hold H.264, HEVC, VP9, AV1,
or many less-common codecs. The codec inside the container is what matters.

### 2. Hardware decoding

The engine creates an FFmpeg `AVHWDeviceContext` of type `D3D11VA`, connected
to the application's D3D11 device and immediate context. FFmpeg is configured
to select `AV_PIX_FMT_D3D11` and decode into GPU surfaces.

The hardware frame context uses:

```text
format:    AV_PIX_FMT_D3D11
software:  AV_PIX_FMT_NV12
size:      original codec width x original codec height
```

This is a zero-copy design for the normal supported path: decoded pixels stay
in GPU memory and are handed directly to the D3D11 video processor. The
renderer does not call `sws_scale()` and does not upload a CPU image every
frame.

### 3. Decoder thread and frame queue

The decoder loop runs on a separate C++ thread. It repeatedly:

1. Reads one packet with `av_read_frame()`.
2. Ignores packets that are not from the selected video stream.
3. Sends video packets to the codec.
4. Receives every available decoded frame.
5. Converts the frame timestamp to seconds.
6. Pushes the frame pointer and timestamp into the bounded queue.

The queue blocks the decoder when it is full. The render thread blocks when it
is empty. This prevents the engine from decoding the whole file immediately.

The queue size is clamped to 3 through 18 frames. The Rust launcher currently
passes `3`, which is a good low-memory default. FFmpeg's hardware frame pool is
set to `queue size + 6`, so the current normal configuration requests at least
nine GPU decode surfaces.

At the end of a file, the engine seeks back to the start to loop it. The code
currently does not drain delayed decoder frames or call
`avcodec_flush_buffers()` after the seek; that is a loop-boundary correctness
item for future work.

### 4. Timing and presentation

The render thread uses `QueryPerformanceCounter` and the decoded frame PTS to
time playback. It:

1. Starts a playback clock from the first frame PTS.
2. Waits until a future frame's PTS is due.
3. Drops a frame that is more than 30 ms late.
4. Detects a PTS jump backwards as a loop restart and resets the clock.
5. Uses `Present(0, 0)` after the video processor writes the frame.

The final short timing interval currently uses a small busy/yield loop. That
helps timing precision but can consume unnecessary CPU time; a waitable timer
would be more power-efficient.

## Where scaling and colour conversion happen

The C++ engine does **not** create a 4K swap chain on a 1080p primary desktop.
It creates the swap chain at the wallpaper window size, which starts from the
primary screen dimensions.

The D3D11 video processor receives two sizes:

```text
Input:  codec width x codec height       (for example 3840 x 2160)
Output: swap-chain width x height        (for example 1920 x 1080)
```

It sets source and destination rectangles, then calls `VideoProcessorBlt()`.
That operation writes the screen-sized result into the BGRA swap-chain back
buffer. It is the place where the engine performs:

- GPU scaling/downscaling;
- NV12/YCbCr to RGB conversion;
- configured colour-space/range handling.

There is one correctness detail to fix later: the destination rectangle is
currently configured for video-processor stream `1`, while the one stream
passed to `VideoProcessorBlt()` is stream `0`. The destination rectangle should
use stream `0`.

## Why 4K still uses more GPU on a 1080p monitor

The app already draws the final image at the monitor/window resolution, but
the scaling happens **after** decoding. A 4K video must still be fully decoded
into 4K surfaces first.

At the same frame rate, 4K has four times the pixels of 1080p:

| Source | Pixels per frame | Pixels decoded per second at 60 FPS |
| --- | ---: | ---: |
| 1920 x 1080 | about 2.07 million | about 124 million |
| 3840 x 2160 | about 8.29 million | about 498 million |

Therefore a higher GPU Video Decode percentage for 4K is expected, even when
the monitor is only 1080p. The GPU must decode the original source before it
can shrink it.

To make 4K videos cost approximately like 1080p videos during normal playback,
the app needs to play a 1080p proxy version, not merely scale a 4K frame at the
end of the pipeline.

## File-format support: current truth

FFmpeg can demux and decode a very broad range of files when its build includes
the required demuxer and decoder. The project links against a "full" shared
FFmpeg build, so opening many containers is possible.

However, the current C++ playback engine is deliberately hardware-only. Its
pixel-format callback accepts only `AV_PIX_FMT_D3D11`; when a decoder cannot
provide that hardware format, it exits. The render path also assumes every
`AVFrame` contains a D3D11 texture, so it cannot display an ordinary software
decoded frame yet.

This gives the following practical result:

| Container example | Can FFmpeg often open it? | Will current live playback work? |
| --- | --- | --- |
| MP4/MOV/MKV with 8-bit H.264 | Yes | Usually, if D3D11VA driver support is available. |
| MP4/MOV/MKV with 8-bit HEVC | Yes | Often, if the GPU supports that HEVC profile. |
| WebM/MKV with VP9 or AV1 | Yes | Only when the user's hardware/driver and FFmpeg D3D11 path support it. |
| ASF/WMV | Often | Depends on the codec inside: H.264 can work; older WMV/VC-1 support varies by GPU/driver. |
| AVI | Often | Depends on the actual codec. Old DivX/Xvid-style codecs commonly have no hardware path. |
| OGV/Ogg with Theora video | Usually | Usually no: FFmpeg software-decodes Theora, but this engine has no software fallback. |
| Audio-only Ogg | Yes as audio | No: the engine requires a video stream. |

So it is safe to say: **the app supports hardware-decodable video codecs, not
every extension FFmpeg understands.** Never advertise an entire container such
as AVI, ASF, or OGG as universally supported.

## Recommended future design

The best long-term design solves both format compatibility and GPU use:

```text
Import selected video
       |
       v
Probe container, codec, dimensions, frame rate, and hardware support
       |
       +--> supported hardware codec and reasonable source size
       |       -> play original through current zero-copy D3D11 path
       |
       +--> unsupported codec, very large source, or high frame rate
               -> make/cache a monitor-resolution H.264 8-bit proxy
               -> play the proxy through the same zero-copy D3D11 path
```

A proxy can use the monitor resolution and a chosen target frame rate, such as
1920x1080 at 30 or 60 FPS. The one-time conversion can use FFmpeg and, when
available, Intel Quick Sync. The original file remains untouched. Later
playback is much cheaper and predictable.

This strategy avoids making a CPU fallback the normal wallpaper path. A CPU
fallback is useful for compatibility, but it can consume much more battery and
CPU for difficult formats.

## Current resource-sensitive areas

These are the most useful places to optimise later:

1. **Source resolution and FPS** — a cached proxy has the largest effect on
   decode workload.
2. **Decode queue size** — keep the normal queue at 3–4 frames rather than
   increasing it for no reason. A 4K NV12 surface is roughly 11.9 MiB, so a
   large hardware frame pool can use substantial shared GPU memory.
3. **Swap-chain buffer count** — the current six BGRA buffers use extra memory
   and add latency. Two or three should be evaluated with correct flip-model
   back-buffer handling.
4. **Per-frame input-view creation** — the video processor currently creates a
   D3D11 input view for every decoded frame. Caching views for the stable decode
   surfaces would reduce CPU/driver overhead.
5. **Presentation pacing** — cap presentation to the monitor refresh rate;
   presenting 120 source frames on a 60 Hz monitor does not make the animation
   visibly smoother.
6. **Resize/display changes** — recreate the swap chain, output view, and
   video processor when the target monitor size changes. The current engine
   only reads its initial dimensions.

## Shutdown note

The C++ window loop calls `std::exit()` after receiving `WM_QUIT`. That ends
the process, and Windows reclaims the process resources, but it bypasses normal
destruction of the local `Engine` object. A future graceful shutdown path should
stop the decoder, wake the queue, join the decoder thread, release FFmpeg/D3D
resources, then return normally from `wWinMain`.

## Useful source files

### C++ engine

- `Engine.cpp` — owns and wires the native playback components.
- `FFmpeg/FFmpeg.cpp` and `FFmpeg/DecoderLoop.inl` — D3D11VA decoder setup and
  decode loop.
- `FFmpeg/FrameQueue.cpp` and `FFmpeg/FramePool.cpp` — bounded decoded-frame
  buffering.
- `DXVA/DXVA.cpp` — D3D11 video-processor setup, colour setup, scaling, and
  `VideoProcessorBlt()`.
- `Render/Render.cpp` and `Render/Render.inl` — PTS clock, late-frame dropping,
  and present.
- `SwapChin/SwapChain.cpp` — composition swap-chain creation.
- `Window/Window.cpp` and `Window/WorkerW.cpp` — wallpaper window placement.

### Rust programs

- `rust/ranami-wallpapers/src/main.rs` — client startup.
- `rust/ranami-wallpapers/src/main_loop.rs` — engine/tray lifecycle and
  wallpaper switching.
- `rust/ranami-wallpapers/src/namepipe.rs` — named-pipe server.
- `rust/ranami-wallpapers/src/engine.rs` — C++ engine process arguments.
- `rust/ranami-wallpapers-gui/src/main.rs` — GUI callbacks and library refresh.
- `rust/ranami-wallpapers-gui/src/init_file_picker.rs` — picker filters.
- `rust/ranami-wallpapers-gui/src/thumbnail_cache.rs` — thumbnail and metadata
  helpers.
- `rust/shared/src/lib.rs` — shared names, paths, messages, and structures.
