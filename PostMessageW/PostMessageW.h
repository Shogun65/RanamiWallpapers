#pragma once
#include <windows.h>
#include <iostream>

class MyPostMessage{
public:

    void postmessage(HWND client_hwnd, 
                    size_t message, 
                    WPARAM wparam, 
                    LPARAM lparam);

    void sentmessage_err(HWND client_hwnd, 
                    WPARAM wparam, 
                    LPARAM lparam);

private:

};