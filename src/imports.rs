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
use std::ffi::*;
pub fn ui_begin_lstr(caption:&str){
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        ui_begin(txt_cstr.as_ptr());
    }

}
pub fn ui_begin_str(caption:String){
    ui_begin_lstr(caption.as_str());
}
pub fn ui_begin_menu_lstr(caption:&str)->bool{
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        return ui_begin_menu(txt_cstr.as_ptr());
    }

}

pub fn ui_begin_menu_str(caption:String)->bool{
    return ui_begin_menu_lstr(caption.as_str());
}
pub fn ui_button_lstr(caption:&str)->bool{
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        return ui_button(txt_cstr.as_ptr());
    }

}

pub fn ui_button_str(caption:String)->bool{
    return ui_button_lstr(caption.as_str());
}
pub fn ui_menu_item_lstr(caption:&str)->bool{
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        return ui_menu_item(txt_cstr.as_ptr());
    }
}
pub fn ui_menu_item_str(caption:String)->bool{
    return ui_menu_item_lstr(caption.as_str());
}
pub fn ui_begin_listbox_lstr(caption:&str,w:i32,h:i32)->bool{
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        return ui_begin_listbox(txt_cstr.as_ptr(),w,h);
    }
}
pub fn ui_begin_listbox_str(caption:String,w:i32,h:i32)->bool{
    return ui_begin_listbox_lstr(caption.as_str(),w,h);
}
pub fn ui_selectable_lstr(caption:&str,is_selected:bool)->bool{
    unsafe{
        let txt_cstr=CString::new(caption).expect("");
        return ui_selectable(txt_cstr.as_ptr(),is_selected);
    }

}
pub fn ui_selectable_str(caption:String,is_selected:bool)->bool{
    return ui_selectable_lstr(caption.as_str(),is_selected);
}