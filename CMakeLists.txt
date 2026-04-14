cmake_minimum_required(VERSION 3.10)

project(AliveWallpaperEngine)

set(CMAKE_CXX_STANDARD 17)

add_executable(AliveWallpaperEngine WIN32
    main.cpp
    Engine.cpp
    DComp/DComp.cpp
    DX/DXDevice.cpp
    DXVA/DXVA.cpp
    FFmpeg/FFmpeg.cpp
    FFmpeg/FramePool.cpp
    FFmpeg/FrameQueue.cpp
    Render/Render.cpp
    SwapChin/SwapChain.cpp
    Window/Window.cpp
    Window/WorkerW.cpp
)

# Include directories
target_include_directories(AliveWallpaperEngine PRIVATE
    ${CMAKE_SOURCE_DIR}
    "C:/Users/gmy87/OneDrive/Documents/codeing lib/ffmpeg-7.1.1-full_build-shared/include"
)

# Link FFmpeg libs
target_link_directories(AliveWallpaperEngine PRIVATE
    "C:/Users/gmy87/OneDrive/Documents/codeing lib/ffmpeg-7.1.1-full_build-shared/lib"
)

target_link_libraries(AliveWallpaperEngine PRIVATE
    avformat
    avcodec
    avutil
    swscale
)

# Windows libs (important for your project)
target_link_libraries(AliveWallpaperEngine PRIVATE
    d3d11
    dxgi
    dcomp
)

target_compile_definitions(AliveWallpaperEngine PRIVATE
    UNICODE
    _UNICODE
)