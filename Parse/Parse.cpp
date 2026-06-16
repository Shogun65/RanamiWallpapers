#include "Parse.h"
#include <iostream>
#include <Windows.h>
#include <shellapi.h>

Parse::ArgsData Parse::get_data()
{
    int argc = 0;

    LPWSTR* argv = CommandLineToArgvW(GetCommandLineW(), &argc);

    std::wcout << argv[1] << L" : "<< argv[2] << L'\n';

    std::wstring video_path = argv[1];
 

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


    return ArgsData{result, std::stoi(argv[2])};

}