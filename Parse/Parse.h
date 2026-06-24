#pragma once

#include <string>
#include <Windows.h>

class Parse
{
public:

    struct ArgsData
    {
        std::string video_path;
        int buffer_count;
        HWND client_hwnd;
    };

    ArgsData get_data();

};