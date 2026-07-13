# How to Use Engine Messages

Engine messages are custom Windows messages used for HWND-to-HWND communication between the native engine side and the Rust-managed side.

## Who uses them

The messages are mainly passed between:

- the native C++ engine window
- the hidden Rust client window
- the tray process

The GUI is different:

- the GUI does not select wallpapers through engine messages
- it sends wallpaper paths to the Rust client through the named pipe instead

## Why messages exist

The engine core is kept fairly focused on wallpaper playback and rendering.

Because of that, higher-level process control is handled outside the engine and sent in through custom messages, for example:

- exit requests
- startup result messages
- "open GUI" style commands
- engine HWND handoff

## Basic example

A component can post a custom message like this:

```cpp
PostMessageW(hwnd, _WM_ENGINE_EXIT, 0, 0);
```

The receiver handles it inside its window procedure or message loop.

## Current message source of truth

The current shared message IDs live in:

- `rust/shared/src/lib.rs`

For the current list, see:

- [engine_message_list.md](engine_message_list.md)

## Message flow

```text
Tray process
      |
      | custom Windows message
      v
Hidden Rust client window
      |
      | custom Windows message
      v
Native engine window
      |
      v
WindowProc / message handler
```

Separate path for wallpaper selection:

```text
GUI double-click
      |
      | named pipe json command
      v
Rust client
      |
      | spawn / restart core process
      v
Native engine process
```

## Important note for future changes

If you are deciding between engine messages and the named pipe:

- use engine messages for HWND/window-level commands
- use the named pipe for higher-level data like "switch to this wallpaper path"
