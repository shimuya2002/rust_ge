use std::ffi::*;
use std::ptr::*;
use crate::app::*;
use crate::game_app::*;

///メインメニューから選択するとアプリケーションを終了させる
/// 'p_user_data' GameAppオブジェクトへのポインタ
pub fn quit_app(p_user_data:*mut c_void){
    unsafe{
        let mut app_obj=p_user_data as *mut GameApp;
        (*app_obj).app.quit();
    }
}
///メインメニューから選択するとダンジョンの状態を再生成する
/// 'p_user_data' GameAppオブジェクトへのポインタ
pub fn reset_floor(p_user_data:*mut c_void){
    unsafe{
        let mut app_obj=p_user_data as *mut GameApp;
        (*app_obj).reset_floor();
    }
}