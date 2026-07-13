# Engine Message List

These values live in `rust/shared/src/lib.rs` under `shared::message`.

`WM_APP` starts at `0x8000`, so every custom engine/client/tray message is built from that base.

## Current messages

- `WM_ENGINE_TEST = WM_APP + 1`
  - simple test message
- `WM_ENGINE_EXIT = WM_APP + 2`
  - tells the engine to exit
- `WM_ENGINE_BOOTUP_SUCCESS = WM_APP + 3`
  - used when the engine boots successfully
- `WM_ENGINE_BOOTUP_FAILED = WM_APP + 4`
  - used when engine startup fails
- `WM_ENGINE_SENT_HWND = WM_APP + 5`
  - engine sent this whit his own HWND
- `WM_ENGINE_OPEN_GUI = WM_APP + 6`
  - used to request opening the GUI
- `WM_ENGINE_D3D11_FMT_NOT_FOUND = WM_APP + 7`
  - used for the "requested D3D11 pixel format was not found" failure path

## Why this file exists

These messages are the small contract between:

- the hidden Rust client window
- the tray process
- the native engine window

If you add a new message:

1. add it in `rust/shared/src/lib.rs`
2. document it here
3. wire the sender and receiver side together
