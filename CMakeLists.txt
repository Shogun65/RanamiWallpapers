cmake_minimum_required(VERSION 3.15)

project(RanamiWallpapers)

set(CMAKE_CXX_STANDARD 17)

add_executable(RanamiWallpapers WIN32
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
    Parse/Parse.cpp
    PostMessageW/PostMessageW.cpp
)

set_property(TARGET RanamiWallpapers PROPERTY
    MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

# Include directories
target_include_directories(RanamiWallpapers PRIVATE
    ${CMAKE_SOURCE_DIR}
    "C:/Users/gmy87/OneDrive/Documents/codeing lib/ffmpeg-7.1.1-full_build-shared/include"
)

# Link FFmpeg libs
target_link_directories(RanamiWallpapers PRIVATE
    "C:/Users/gmy87/OneDrive/Documents/codeing lib/ffmpeg-7.1.1-full_build-shared/lib"
)

target_link_libraries(RanamiWallpapers PRIVATE
    avformat
    avcodec
    avutil
    swscale
)

# Windows libs
target_link_libraries(RanamiWallpapers PRIVATE
    d3d11
    dxgi
    dcomp
    Ole32
)

target_compile_definitions(RanamiWallpapers PRIVATE
    UNICODE
    _UNICODE
)

# Force release mode optimizations
set(CMAKE_BUILD_TYPE Release)

# Optional: extra optimization flags
add_compile_options(/O2 /GL)
add_link_options(/LTCG)