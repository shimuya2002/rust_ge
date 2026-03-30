cmake_minimum_required(VERSION 3.5 FATAL_ERROR)
project(SDL2_FFI_LIB VERSION 1.0)

#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")

if(APPLE)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")
endif(APPLE)
if(WIN32)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -utf-8 -std:c++20 ")
SET(CMAKE_EXE_LINKER_FLAGS  "${CMAKE_EXE_LINKER_FLAGS} -NODEFAULTLIB:LIBCMT")
endif(WIN32)

if(APPLE)
    include_directories(/opt/local/include/)
    include_directories(/opt/local/include/SDL2)
    link_directories(/opt/local/lib)
    link_libraries(SDL2)
    link_libraries(SDL2_image)
    link_libraries(SDL2_gfx)
    link_libraries(SDL2_ttf)
    link_libraries(SDL2main)
    link_libraries(lua5.4)

endif(APPLE)
if(WIN32)
    include_directories("D:/Program Files/SDL2/SDL2-2.24.1/include")
    include_directories("D:/lib/SDL2_gfx-main")
    include_directories("D:/lib/SDL2_image-devel-2.6.3-VC/SDL2_image-2.6.3/include")
    include_directories("D:/lib/SDL2_ttf-2.20.2/include")
    include_directories("D:/lib/lua-5.4.2_Win64_vc16_lib/include")
    link_directories("D:/Program Files/SDL2/SDL2-2.24.1/lib/x64")
    link_directories("D:/lib/lua-5.4.2_Win64_vc16_lib")
    link_directories("D:/lib/SDL2_image-devel-2.6.3-VC/SDL2_image-2.6.3/lib/x64")
    link_directories("D:/lib/SDL2_ttf-2.20.2/lib/x64")
    link_directories("D:/lib/SDL2_gfx-main/build/Debug")
endif(WIN32)
include_directories(./imgui/)
include_directories(./imgui/backends/)

add_library(SDL2_ffi
    ./src/app.cpp 
#    window.cpp 
#    renderer.cpp 
#    texture.cpp 
#    font.cpp
#    RWOps.cpp
    ./imgui/imgui_draw.cpp
    ./imgui/imgui_tables.cpp 
    ./imgui/imgui_widgets.cpp 
    ./imgui/imgui.cpp 
    ./imgui/backends/imgui_impl_sdl2.cpp 
    ./imgui/backends/imgui_impl_sdlrenderer2.cpp
    ./imgui/misc/cpp/imgui_stdlib.cpp

 #   gpage.cpp
 #   anim_frame.cpp
#    petit_com_lua.cpp
#    rpg_maker_ui.cpp
   #show_actor.cpp
#    show_armor.cpp
#    show_enemy.cpp
#    show_event.cpp
#    show_item.cpp
#    show_job.cpp
#    show_map.cpp
#    show_skill.cpp
#    show_state.cpp
#    show_type.cpp
#    show_tileset.cpp
#    show_weapon.cpp
)
