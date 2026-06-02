use std::ffi::*;
use std::ptr::*;
use std::env::*;
use std::rc::*;

use std::collections::*;
use crate::app::*;
use crate::config::*;
use crate::imports::*;
use crate::ui_item::*;
use crate::ui_menu::*;
use crate::ui_window::*;
use crate::ui_menubar::*;
use crate::ui_popup::*;
//use crate::game_app_event::*;
use crate::sb_state::*;
use crate::sb_cmdtype::*;
use crate::sb_syntax::*;
use crate::geometory::*;
use crate::rouge_dungeon::*;
use crate::sprite::*;
use crate::adv::*;
use crate::animation::*;
include!("./geometory_inc.rs");
//ゲームモード

const MODE_ADV:usize=0;
const MODE_DUNGEON:usize=MODE_ADV+1;
const BG_IMAGE:usize=0;
const LEFT_BUSTUP_IMAGE:usize=1;
const RIGHT_BUSTUP_IMAGE:usize=2;
const MSG:usize=3;
const PLAYER:usize=4;

#[derive(PartialEq)]
enum DLG_MODE{
    NONE,
    SHARE,
    LOCAL,
    GPAGES(usize),
    SPRITES(usize),
    ANIMATIONS(usize,usize)

}

///BASICアプリ　オブジェクト
pub struct BasicApp{
    ///* 'app' アプリオブジェクト
    pub app:App,
    ///管理するスプライト
    pub sprites:Vec<Rc<Sprite>>, 
    ///管理するアニメーション
    pub animations:Vec<Rc<Animation>>,
    ///実行するスクリプト   
    script:Option<SB_State>,
    pub mode:usize,
    adv:Adv,
    dungeon:RougeDungon,
    ///スクリプトから呼び出させる関数の定義テーブル
    script_types:HashMap<String,TypeInfo>,
    dlg_mode:DLG_MODE 
}
impl BasicApp{
    pub fn new()->Self{
        //スクリプトから呼ばれる定義を設定
        let mut scr_types=HashMap::new();
        scr_types.insert(String::from("print"),TypeInfo::NativeFunc(Some(print)));
        scr_types.insert(String::from("create_gpages"),TypeInfo::NativeFunc(Some(create_gpages)));

        scr_types.insert(String::from("load_image"),TypeInfo::NativeFunc(Some(load_image)));
        scr_types.insert(String::from("create_sprite"),TypeInfo::NativeFunc(Some(create_sprite)));
        scr_types.insert(String::from("set_sprite"),TypeInfo::NativeFunc(Some(set_sprite)));
        scr_types.insert(String::from("copy_sprite"),TypeInfo::NativeFunc(Some(copy_sprite)));
        scr_types.insert(String::from("set_msg"),TypeInfo::NativeFunc(Some(msg)));
        scr_types.insert(String::from("set_mode"),TypeInfo::NativeFunc(Some(set_mode)));
        scr_types.insert(String::from("create_dungeon"),TypeInfo::NativeFunc(Some(create_dungeon)));
        scr_types.insert(String::from("mode_adv"),TypeInfo::Const(VarType::Int(MODE_ADV as i32)));
        scr_types.insert(String::from("mode_dungeon"),TypeInfo::Const(VarType::Int(MODE_DUNGEON as i32)));
        scr_types.insert(String::from("bg_image"),TypeInfo::Const(VarType::Int(BG_IMAGE as i32)));
        scr_types.insert(String::from("left_bustup_image"),TypeInfo::Const(VarType::Int(LEFT_BUSTUP_IMAGE as i32)));
        scr_types.insert(String::from("right_bustup_image"),TypeInfo::Const(VarType::Int(RIGHT_BUSTUP_IMAGE as i32)));
        scr_types.insert(String::from("msg"),TypeInfo::Const(VarType::Int(MSG as i32)));
        scr_types.insert(String::from("player"),TypeInfo::Const(VarType::Int(PLAYER as i32)));

        return Self{
            app:App::new(),
            sprites:Vec::new(),
            animations:Vec::new(),
            script:None,
            mode:MODE_ADV,
            adv:Adv::new(),
            dungeon:RougeDungon::new(),
            script_types:scr_types,
            dlg_mode:DLG_MODE::NONE,
        };
    }
    pub fn on_init(&mut self){
        if 1<args().len(){

            if let Err(msg)=self.load_script(args().nth(1).unwrap()){
                println!("Error:{}",msg);
            }
        }
        self.dungeon.set_render_rect(&rect_type!{0,0,WND_W,WND_H});     
    }
    fn load_script(&mut self,file_path:String)->Result<(),String>{
        let r=SB_State::load_from_file(&file_path,Some(&self.script_types));
        if let Ok(mut s)=r{
            s.dump();
            s.share_var_tbl.insert(String::from("result"),VarType::Bool(false));
            self.script=Some(s);
            
        }else if let Err(msg)=r{
            return Err(msg);
        }

        return Ok(());
    }
    pub fn update(&mut self){
        
        
        if DLG_MODE::NONE== self.dlg_mode{
            match self.mode{
                MODE_ADV=>{
                    self.adv.update(&mut self.app);
                },
                MODE_DUNGEON=>{
                    self.dungeon.update(&mut self.app);
                },
                _=>{
                    assert!(false);
                }
            }
            if self.check_script_can_run(){
                let p_ud=self as *mut _ as *mut c_void;
                if let Some(s)=&mut self.script{
                    s.p_user_data=p_ud;
                    if let Some(e_msg)=&s.is_error_raised{
                        self.app.quit();
                        return ;
                    }
                    s.run_script=true;
                    s.run();
                }

            }
                
        }
        
    }
    pub fn paint(&mut self){
        match self.mode{
            MODE_ADV=>{
                self.adv.render(&mut self.app);
            },
            MODE_DUNGEON=>{
                self.dungeon.render(&mut self.app);
            },
            _=>{
                assert!(false);
            }
        }
        self.app.set_gpage(0,0);
        unsafe{
            ui_begin_frame();
            if ui_begin_main_menubar(){
                if ui_begin_menu_lstr("File"){
                    if ui_menu_item_lstr("Exit"){
                        self.app.quit();
                    }

                    ui_end_menu();

                }
                if ui_begin_menu_lstr("Debug"){
                    if ui_menu_item_lstr("Share"){
                        self.dlg_mode=DLG_MODE::SHARE;
                    }
                    if ui_menu_item_lstr("Local"){
                        self.dlg_mode=DLG_MODE::LOCAL;
                    }

                    if ui_menu_item_lstr("GPages"){
                        self.dlg_mode=DLG_MODE::GPAGES(0);
                    }
                    if ui_menu_item_lstr("Sprites"){
                        self.dlg_mode=DLG_MODE::SPRITES(0);
                    }
                    if ui_menu_item_lstr("Animations"){
                        self.dlg_mode=DLG_MODE::ANIMATIONS(0,0);
                    }
                    ui_end_menu();

                }
                ui_end_main_menubar();
                
            }
            match self.dlg_mode{
                DLG_MODE::SHARE=>{
                    ui_begin_lstr("Share variables");
                        if let Some(script)=&self.script{
                            ui_begin_listbox_lstr("##ShareList",127,0);
                            for k in script.share_var_tbl.keys(){
                                let value=&script.share_var_tbl[k];
                                if ui_selectable_str(format!("{}={}",k,value),false){
                                   
                                }
                            }
                            ui_end_listbox();

                        }
                    ui_end();
                },
                DLG_MODE::LOCAL=>{

                },
                DLG_MODE::GPAGES(cur_idx)=>{
                    ui_begin_lstr("Gpages");
                        ui_begin_listbox_lstr("##GPageList",127,0);
                            let mut sel_idx=cur_idx;
                            for i in 0..self.app.g_pages.len(){
                                let is_selected=i==cur_idx;
                                if ui_selectable_str(format!("{}",i),is_selected){
                                    sel_idx=i;
                                }
                            }
                        ui_end_listbox();
                        self.dlg_mode=DLG_MODE::GPAGES(sel_idx);
                        ui_sameline();
                        ui_image(self.app.g_pages[sel_idx],320,240);
                        ui_newline();
                        if ui_button_lstr("OK"){
                            self.dlg_mode=DLG_MODE::NONE;
                        }

                    ui_end();

                },
                DLG_MODE::SPRITES(cur_idx)=>{
                    ui_begin_lstr("Sprites");
                        let mut sel_idx=cur_idx;
                        if ui_begin_listbox_lstr("##SpriteList",127,0){
                            
                            for i in 0..self.sprites.len(){
                                let is_selected=i==cur_idx;
                                if ui_selectable_str(format!("{}",i),is_selected){
                                    sel_idx=i;
                                }
                            }
                            ui_end_listbox();
                        }
                        self.dlg_mode=DLG_MODE::SPRITES(sel_idx);
                        ui_sameline();
                        let sprite=&self.sprites[sel_idx];
                        

                        ui_subimage(self.app.g_pages[sprite.gpage],
                            sprite.src_rect.w as i32,
                            sprite.src_rect.h as i32,
                            (sprite.src_rect.x/ WND_W as f32) ,
                            (sprite.src_rect.y/ WND_H as f32) ,
                            ((sprite.src_rect.x+sprite.src_rect.w) / WND_W as f32),
                            ((sprite.src_rect.y+sprite.src_rect.h) / WND_H as f32)
                        );
                        ui_newline();
                        if ui_button_lstr("OK"){
                            self.dlg_mode=DLG_MODE::NONE;
                        }

                    ui_end();

                },
                DLG_MODE::ANIMATIONS(cur_idx,cur_frame)=>{
                    ui_begin_lstr("Animations");
                        let mut sel_idx=cur_idx;
                        let mut sel_frame=cur_frame;
                        if ui_begin_listbox_lstr("##AnimationList",127,0){
                            
                            for i in 0..self.animations.len(){
                                let is_selected=i==cur_idx;
                                if ui_selectable_str(format!("{}",i),is_selected){
                                    sel_idx=i;
                                }
                            }
                            ui_end_listbox();
                        }
                        if cur_idx!=sel_idx{
                            sel_frame=0;                            
                        }


                        ui_sameline();
                        if sel_idx < self.animations.len(){
                            let animation=&self.animations[sel_idx];
                            if 0<animation.frames.len(){
                                let sprite=&animation.frames[sel_frame];
                                

                                ui_subimage(self.app.g_pages[sprite.gpage],
                                    sprite.src_rect.w as i32,
                                    sprite.src_rect.h as i32,
                                    (sprite.src_rect.x/ WND_W as f32) ,
                                    (sprite.src_rect.y/ WND_H as f32) ,
                                    ((sprite.src_rect.x+sprite.src_rect.w) / WND_W as f32),
                                    ((sprite.src_rect.y+sprite.src_rect.h) / WND_H as f32)
                                );

                            }
                            if sel_frame+1 ==animation.frames.len(){
                                sel_frame=0;
                            }else{
                                sel_frame=sel_frame+1;
                            }

                        }
                        ui_newline();
                        self.dlg_mode=DLG_MODE::ANIMATIONS(sel_idx,sel_frame);
                        if ui_button_lstr("OK"){
                            self.dlg_mode=DLG_MODE::NONE;
                        }
                        
                    ui_end();
                },
                _=>{}
            }
            ui_end_frame();

        }
    }
    fn check_script_can_run(&self)->bool{
        return match self.mode{
            MODE_ADV=>{
               self.adv.update_finished() 
            },
            MODE_DUNGEON=>{
                self.dungeon.update_finished() 
            },
            _=>{
                assert!(false);
                false
            }
        }
    }
    fn proc_user_input(&mut self){
        if self.app.click(){
            let ud=self as *mut _ as  *mut c_void;
            let click_pos=self.app.click_pos();
            match self.mode{
                MODE_ADV=>{
                    self.adv.proc_user_click(click_pos);
                },
                MODE_DUNGEON=>{
                },
                _=>{
                    assert!(false);
                }
            } 
        }
    }
}
fn print(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let msg=state.value_to_string(0);
        (*app).app.log_str_info(&msg);
        return None;
    }
}

fn create_gpages(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let num=state.value_to_int(0) as usize;
        (*app).app.create_gpages(num,WND_W,WND_H);
        (*app).app.log_str_info(&format!("{} gpages created.",num));
        return None;
    }
}
fn load_image(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let path=state.value_to_string(0);
        let target_sprite=state.value_to_int(1) as usize;
        let sprite=(&(*app).sprites)[target_sprite].clone();

        (*app).app.set_gpage(sprite.gpage,RENDER_GPAGE);
        let res=(*app).app.load_image(sprite.src_rect.x as i32,
                                    sprite.src_rect.y as i32,
                                    path.as_str());
        if let Err(e_msg)=res{
            state.is_error_raised=Some(e_msg);
        }
        return None;
    }
}
fn create_sprite(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let height=state.value_to_int(0);
        let width=state.value_to_int(1);
        let y=state.value_to_int(2);
        let x=state.value_to_int(3);
        let page=state.value_to_int(4);
        let sprite=Sprite{
            src_rect:rect_type!{x,y,width,height},
            gpage:page as usize,
            render_mode:GPageRenderMode::Norm,
        };
        (*app).sprites.push(Rc::new(sprite));
        (*app).app.log_str_info(
            &format!("Sprite created page={},[{},{},{},{}]",
                page,x,y,width,height));
        return Some(VarType::Int(((*app).sprites.len()-1) as i32));
    }
}
///スプライトを設定する
fn set_sprite(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let idx=state.value_to_int(0) as usize;
        let target=state.value_to_int(1) as usize;

        if idx < (*app).sprites.len(){
            let sprite=(&(*app).sprites)[idx].clone();
            match target{
                BG_IMAGE=>{
                    (*app).adv.set_bg_image(Some(sprite.clone()));
                    (*app).app.log_str_info(
                         &format!("Sprite set target=BG_IMAGE,{}",sprite.clone()));
                },
                LEFT_BUSTUP_IMAGE=>{
                    (*app).adv.set_left_bustup_image(Some(sprite.clone()));
                    (*app).app.log_str_info(
                         &format!("Sprite set target=LEFT_BUSTUP_IMAGE,{}",sprite.clone()));
                },
                RIGHT_BUSTUP_IMAGE=>{
                    (*app).adv.set_right_bustup_image(Some(sprite.clone()));
                    (*app).app.log_str_info(
                         &format!("Sprite set target=RIGHT_BUSTUP_IMAGE,{}",sprite.clone()));
                },
                MSG=>{
                    (*app).adv.set_msg_rect(Some(sprite.clone()));
                    (*app).app.log_str_info(
                         &format!("Sprite set target=MSG,{}",sprite.clone()));

                },
                PLAYER=>{
                    (*app).dungeon.set_player_image(Some(sprite.clone()));
                     (*app).app.log_str_info(
                         &format!("Sprite set target=PLAYER,{}",sprite.clone()));
                },
                _=>{
                    state.is_error_raised=Some(
                        format!("Invalid image set {}",target)
                    );
                }
            }


        }else{
            state.is_error_raised=Some(
                format!("Sprites available index is 0 < {} not {}",
                    (*app).sprites.len(),
                    idx
                )
            );

        }
        

        return None;
    }
}

fn copy_sprite(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let dst=state.value_to_int(0) as usize;
        let src=state.value_to_int(1) as usize;  
        if src > (*app).sprites.len(){
            state.is_error_raised=Some(
                format!("Sprites available index is 0 < {} not {}",
                    (*app).sprites.len(),
                    src
                )
            );

        }
        
        if dst > (*app).sprites.len(){
            state.is_error_raised=Some(
                format!("Sprites available index is 0 < {} not {}",
                    (*app).sprites.len(),
                    src
                )
            );

        }
        (&mut ((*app).sprites))[dst]=(&mut ((*app).sprites))[src].clone();
        return None;

    }
}
fn msg(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let msg=state.value_to_string(0);
        (*app).adv.set_msg(msg);
        return None;
    }
}

fn set_mode(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        let mode=state.value_to_int(0) as usize;
        if mode!=MODE_ADV && mode!=MODE_DUNGEON{
            state.is_error_raised=Some(
                format!("Invalid value {}",
                    mode
                )
            );
        }
        (*app).mode=mode;

        return None;
    }
}
fn create_dungeon(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut BasicApp;
        (*app).dungeon.reset();
        return None;
    }
}