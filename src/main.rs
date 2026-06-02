#![allow(warnings)]
mod imports;
mod app;
mod config;
mod texture;
mod cache_tbl;
mod sb_lexer;
mod sb_syntax;
mod sb_cmdtype;
mod rw_file;
mod sb_state;
//mod script_func;
//mod game_app;
mod font;
mod geometory;
mod ui_item;
mod ui_menu;
mod ui_window;
mod ui_menubar;
mod ui_popup;

//mod game_app_event;
mod rouge_dungeon;
mod sprite;
mod adv;
mod basic_app;
mod animation;
use std::ffi::*;

use crate::imports::*;
use crate::app::*;
use crate::config::*;
use crate::sb_syntax::*;
use crate::sb_cmdtype::*;
//use crate::game_app::*;
use crate::sb_state::*;
use crate::basic_app::*;


fn on_game_init(p:*mut c_void){
    unsafe{
        let mut app_obj=p as *mut BasicApp;
        (*app_obj).on_init();
    }
}
fn on_game_term(p:*mut c_void){
    unsafe{
        let mut app_obj=p as *mut BasicApp;
    }
}
fn main() {
    println!("Hello, world!");
    let mut app_obj=BasicApp::new();//GameApp::new();
    if cfg!(feature="non_bindings"){
        unsafe{
            let p_ud=&mut app_obj as *mut _ as *mut c_void;
            app_obj.app.set_ud(p_ud);
            app_obj.app.set_init_event(on_game_init);
            app_obj.app.set_term_event(on_game_term);

        }
    }else{
        unsafe{
 # [cfg(not(feature="non_bindings"))]
           let p_app=app_obj.app.p_app;
 # [cfg(not(feature="non_bindings"))]
            set_ud(p_app,&mut app_obj as *mut _ as *mut c_void);
 # [cfg(not(feature="non_bindings"))]
            set_on_paint(p_app,Some(on_paint));
 # [cfg(not(feature="non_bindings"))]
            set_on_init(p_app,Some(on_init));
 # [cfg(not(feature="non_bindings"))]
            set_on_quit(p_app,Some(on_quit));
        }
    }
//    let mut script=None;
    
    while app_obj.app.run_step(WND_W,WND_H){
       /* if let Some(ref mut scr)=script{
            scr.run();
//            scr.run_script=app_obj.render();
            scr.run_script=app_obj.update();
            app_obj.paint();
        }*/
        app_obj.update();
        app_obj.paint();

        
    }

}
