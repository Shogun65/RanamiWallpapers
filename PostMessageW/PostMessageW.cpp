#include "PostMessageW.h"

void MyPostMessage::postmessage(
    HWND client_hwnd,
    size_t message,
    WPARAM wparam = 0,
    LPARAM lparam = 0) 
{

    //uint32_t WM_ENGINE_SENT_HWND = WM_APP + 5;

    std::cout 
            << "Msg of postmessage: " << message << '\n'
            << "wparam: " << wparam << '\n'
            << "lparam: " << lparam << '\n';

    bool result = PostMessageW(
        client_hwnd,
        message,
        wparam,
        lparam
    );

    if (!result) {
        MessageBoxW(nullptr, L"Error on PostMessageW", L"Error", MB_ICONERROR);
    }
}

void MyPostMessage::sentmessage_err(
    HWND client_hwnd, 
    WPARAM wparam = 0, 
    LPARAM lparam = 0)
{                                     
    UINT const WM_ENGINE_BOOTUP_FAILED = WM_APP + 4;

    std::cout 
            << "Msg of sentmessage: " << WM_ENGINE_BOOTUP_FAILED << '\n'
            << "wparam: " << wparam << '\n'
            << "lparam: " << lparam << '\n'; 

    bool result = SendMessageW(client_hwnd, WM_ENGINE_BOOTUP_FAILED, wparam, lparam);

    if (!result) {
        MessageBoxW(nullptr, L"Error on SendMessageW", L"Error", MB_ICONERROR);
    }
}