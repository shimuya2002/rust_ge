use std::ffi::*;
use std::ptr::*;
use crate::app::*;
use crate::game_app::*;
pub fn quit_app(p_user_data:*mut c_void){
    unsafe{
        let mut app_obj=p_user_data as *mut GameApp;
        (*app_obj).app.quit();
    }
}