#include "Parse.h"
#include <iostream>
#include <Windows.h>
#include <shellapi.h>

Parse::ArgsData Parse::get_data()
{
    int argc = 0;

    LPWSTR* argv = CommandLineToArgvW(GetCommandLineW(), &argc);

    std::wcout << L" Args: " << argv[1] << L" : "<< argv[2] << L" : " << argv[3] << L'\n';

    std::wstring video_path = argv[1];
 
    // you know what i forget how it work but it not like iam going to touch this anyway haha!
    int size = WideCharToMultiByte(
        CP_UTF8,
        0,
        argv[1],
        -1,
        nullptr,
        0,
        nullptr,
        nullptr
    );

    std::string result(size - 1, '\0');

    WideCharToMultiByte(
        CP_UTF8,
        0,
        argv[1],
        -1,
        result.data(),
        size,
        nullptr,
        nullptr 
    );

    uintptr_t hwnd = _wcstoui64(argv[3], nullptr, 0);
    std::cout << "hwnd_of_arg_after_convert: " << hwnd << '\n';
    
    HWND client_hwnd = reinterpret_cast<HWND>(hwnd);

    std::cout << "client hwnd in reinterpret_cast: " << client_hwnd << '\n';

    return ArgsData{result, std::stoi(argv[2]), client_hwnd};

}