#include "Window.h"
#include <iostream>

/*
* This is Funs Enable Bebug console
* You can Print anythink inside it by use
* Printf or wPrintf (I recommend)
* You can Print Anywhere as you want to see if thinks
* Values or print error and mean print really error
* Not just (example) CreateWindowEx fail but why it fail
* Print the Error code of HRESULT or GetLastError and google it what the error code means
*/
void Window::InitDebugConsole()
{
    AllocConsole();

    FILE* f;
    freopen_s(&f, "CONOUT$", "w", stdout);
    freopen_s(&f, "CONOUT$", "w", stderr);
    freopen_s(&f, "CONIN$", "r", stdin);

    // Disable buffering so prints from other threads appear immediately
    setvbuf(stdout, NULL, _IONBF, 0);
    setvbuf(stderr, NULL, _IONBF, 0);
     
    printf("----Debug console----\n");
    printf("----EXE VERSION: V0.06----\n");
    fflush(stdout);
}

/*
* This Handel Our Window Events. we can hendel our own events
* Like WM_DESTROY etc.
*/
LRESULT CALLBACK Window::WindowProc(HWND hwnd, UINT umsg, WPARAM wparam, LPARAM lparam)
{
    switch(umsg)
    {
        case WM_DESTROY:
        {
            PostQuitMessage(0);
            return 0;
        }
    }
    return DefWindowProcW(hwnd, umsg, wparam, lparam);
}

/*
* This Funs Return our window HWND
* Why we need HWND. For SwapChin1
* Somethink like this:
* 
* hr = g_DCompDevice->CreateTargetForHwnd(
*        hwnd, <-- see here
*        TRUE,
*        &g_DCompTarget
*    );
* 
*/
HWND Window::GetHWND() const
{
    return _hwnd;
}

/*
* 
* 
* 
* 
* 
*/
bool Window::CreateMainWindow(HINSTANCE hInstance)
{
    WNDCLASSEX wc = { };

    wc.cbSize = sizeof(WNDCLASSEX);
    wc.hInstance = hInstance;
    wc.lpszClassName = L"LiveWallpaperWindow";
    wc.lpfnWndProc = WindowProc;
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);

    RegisterClassEx(&wc);

        _hwnd = CreateWindowEx(
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW,
        wc.lpszClassName,
        L"",
        WS_POPUP,
        0, 0,
        GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN),
        nullptr,
        nullptr,
        hInstance,
        nullptr
    );

    if(!_hwnd)
    {
        DWORD err = GetLastError();
        wprintf(L"CreateWindowEx failed. Error code: %lu\n", err);
        return false;
    }

    //This is For us To know the window size Thats it
    RECT rc;
    GetClientRect(_hwnd, &rc);
    _WindowWidth = rc.right - rc.left;
    _WindowHeight = rc.bottom - rc.top;
    wprintf(L"Window: %d x %d\n", _WindowWidth, _WindowHeight);

    return true;  
}

void Window::AttachHwndToWorkerW(HWND WorkerW)
{
    SetParent(_hwnd, WorkerW);

    RECT rc = { };
    GetClientRect(WorkerW, &rc);

    SetWindowPos(
        _hwnd,
        nullptr,
        0, 0,
        rc.right - rc.left,
        rc.bottom - rc.top,
        SWP_NOACTIVATE | SWP_SHOWWINDOW
    );
    printf("WorkerW: %d x %d\n", rc.right -rc.left, rc.bottom-rc.top);
}

void Window::ShowMainWindow()
{
    ShowWindow(_hwnd, SW_SHOW);
}

LONG Window::GetWindowWidth() const 
{
    return _WindowWidth;
}

LONG Window::GetWindowHeight() const
{
    return _WindowHeight;
}