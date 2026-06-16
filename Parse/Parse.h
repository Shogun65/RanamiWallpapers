#pragma once

#include <string>

class Parse
{
public:

    struct ArgsData
    {
        std::string video_path;
        int buffer_count;
    };

    ArgsData get_data();

};