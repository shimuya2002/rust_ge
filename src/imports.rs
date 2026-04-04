#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/cpp_imports.rs"));

 # [cfg(feature="use_sdl3")]
pub const SDL_WindowFlags_SDL_WINDOW_OPENGL:SDL_WindowFlags=0x0000000000000002;
 # [cfg(feature="use_sdl3")]
pub const SDL_INIT_EVERYTHING:SDL_InitFlags=SDL_INIT_AUDIO|
                                            SDL_INIT_VIDEO|
                                            SDL_INIT_JOYSTICK|
                                            SDL_INIT_HAPTIC|
                                            SDL_INIT_GAMEPAD|
                                            SDL_INIT_EVENTS|
                                            SDL_INIT_SENSOR|
                                            SDL_INIT_CAMERA;

 # [cfg(feature="use_sdl3")]
pub const SDL_QUIT_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_EVENT_QUIT;
 # [cfg(feature="use_sdl2")]
pub const SDL_QUIT_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_QUIT;
 # [cfg(feature="use_sdl3")]
pub const SDL_KEYUP_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_EVENT_KEY_UP;
 # [cfg(feature="use_sdl2")]
pub const SDL_KEYUP_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_KEYUP;

 # [cfg(feature="use_sdl3")]
pub const SDL_KEYDOWN_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_EVENT_KEY_DOWN;
 # [cfg(feature="use_sdl2")]
pub const SDL_KEYDOWN_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_KEYDOWN;

 # [cfg(feature="use_sdl3")]
pub const SDL_MOUSE_BUTTON_DOWN_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_EVENT_MOUSE_BUTTON_DOWN;
 # [cfg(feature="use_sdl2")]
pub const SDL_MOUSE_BUTTON_DOWN_EVENT_VALUE:SDL_EventType=SDL_EventType_SDL_MOUSEBUTTONDOWN;

 # [cfg(feature="use_sdl3")]
pub const SDL_K_UP_VALUE:SDL_Keycode=SDLK_UP;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_UP_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_UP;

 # [cfg(feature="use_sdl3")]
pub const SDL_K_DOWN_VALUE:SDL_Keycode=SDLK_DOWN;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_DOWN_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_DOWN;


 # [cfg(feature="use_sdl3")]
pub const SDL_K_LEFT_VALUE:SDL_Keycode=SDLK_LEFT;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_LEFT_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_LEFT;

 # [cfg(feature="use_sdl3")]
pub const SDL_K_RIGHT_VALUE:SDL_Keycode=SDLK_RIGHT;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_RIGHT_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_RIGHT;

 # [cfg(feature="use_sdl3")]
pub const SDL_K_z_VALUE:SDL_Keycode=SDLK_Z;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_z_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_z;

# [cfg(feature="use_sdl3")]
pub const SDL_K_x_VALUE:SDL_Keycode=SDLK_X;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_x_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_x;

# [cfg(feature="use_sdl3")]
pub const SDL_K_a_VALUE:SDL_Keycode=SDLK_A;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_a_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_a;

# [cfg(feature="use_sdl3")]
pub const SDL_K_s_VALUE:SDL_Keycode=SDLK_S;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_s_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_s;

# [cfg(feature="use_sdl3")]
pub const SDL_K_q_VALUE:SDL_Keycode=SDLK_Q;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_q_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_q;

# [cfg(feature="use_sdl3")]
pub const SDL_K_w_VALUE:SDL_Keycode=SDLK_W;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_w_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_w;

# [cfg(feature="use_sdl3")]
pub const SDL_K_1_VALUE:SDL_Keycode=SDLK_1;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_1_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_1;

# [cfg(feature="use_sdl3")]
pub const SDL_K_2_VALUE:SDL_Keycode=SDLK_2;
 # [cfg(feature="use_sdl2")]
pub const SDL_K_2_VALUE:SDL_Keycode=SDL_KeysymType_SDLK_2;
