cmake_minimum_required(VERSION 3.5 FATAL_ERROR)
project(SDL_FFI_LIB VERSION 1.0)

#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")
if(USE_SDL2)
    set(SDL_USE_VER "USE_SDL2")
endif(USE_SDL2)
if(USE_SDL3)
    set(SDL_USE_VER "USE_SDL3")
endif(USE_SDL3)

if(APPLE)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -D ${SDL_USE_VER}")
endif(APPLE)
if(WIN32)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -utf-8 -std:c++20 -D ${SDL_USE_VER}")
SET(CMAKE_EXE_LINKER_FLAGS  "${CMAKE_EXE_LINKER_FLAGS} -NODEFAULTLIB:LIBCMT")
endif(WIN32)

if(APPLE)
    include_directories(/opt/local/include/)
    include_directories(/opt/local/include/SDL2)
    link_directories(/opt/local/lib)

endif(APPLE)
if(WIN32)
    include_directories("C:\\Users\\shimu\\Documents\\SDL3-3.4.2\\include")
    include_directories("C:\\Users\\shimu\\Documents\\SDL3_image-3.4.0\\include")
    include_directories("C:\\Users\\shimu\\Documents\\SDL3_ttf-3.1.0\\include")
    include_directories("C:\\Users\\shimu\\Documents\\SDL3_gfx-master")
    link_directories("C:\\Users\\shimu\\Documents\\library")
endif(WIN32)
include_directories(./imgui/)
include_directories(./imgui/backends/)
if(USE_SDL2)
add_library(SDL2_ffi
    ./src/imports.cpp
    ./imgui/imgui_draw.cpp
    ./imgui/imgui_tables.cpp 
    ./imgui/imgui_widgets.cpp 
    ./imgui/imgui.cpp 

    ./imgui/backends/imgui_impl_sdl2.cpp 
    ./imgui/backends/imgui_impl_sdlrenderer2.cpp
    ./imgui/misc/cpp/imgui_stdlib.cpp
)
endif(USE_SDL2)

if(USE_SDL3)
add_library(SDL3_ffi
    ./src/imports.cpp
    ./imgui/imgui_draw.cpp
    ./imgui/imgui_tables.cpp 
    ./imgui/imgui_widgets.cpp 
    ./imgui/imgui.cpp 

    ./imgui/backends/imgui_impl_sdl3.cpp 
    ./imgui/backends/imgui_impl_sdlrenderer3.cpp
    ./imgui/misc/cpp/imgui_stdlib.cpp
)
endif(USE_SDL3)

