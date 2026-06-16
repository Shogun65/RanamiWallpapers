# How to Use Engine Messages

## What are Engine Messages?

Engine Messages are custom Windows messages used for communication between the following components:

* Core Engine
* Client
* Tray Application

The Core Engine is intentionally simple. It only needs:

* Video path
* Buffer count (defaults to 3 if not provided)

The engine does not know or care about the rest of the application. Because of this, we use Engine Messages to send commands such as exiting the engine or performing other engine-related actions.

---

## How It Works

A component sends a custom Windows message to the engine window using `PostMessageW()`.

Example:

```cpp
PostMessageW(hwnd, _WM_ENGINE_EXIT, 0, 0);
```

The engine receives the message inside `WindowProc()`.

See:

* `Window/Window.cpp`
* `engine_message_list.md`

---

## Handling Messages

Messages are processed inside a `switch` statement in `WindowProc()`.

Example:

```cpp
case _WM_ENGINE_EXIT:
{
    PostQuitMessage(0);
    return 0;
}
```

`_WM_ENGINE_EXIT` currently has the value `0x8002` (`WM_APP + 2`).

See:

* `Window/Window.h`
* `engine_message_list.md`

---

## Message Flow

```text
Tray
      |
      | PostMessageW(...)
      v
Client Window (Hidden HWND) <-- See Client Manage everythink (in future he also Manage GUI!)
      |
      |
      v 
Engine Window (HWND)
      |
      v
WindowProc()
      |
      v
Handle Custom Message
```
