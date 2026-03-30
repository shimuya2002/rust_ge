use std::ffi::*;

use crate::app::*;
use crate::config::*;
use crate::imports::*;
use crate::ui_item::*;
use crate::ui_menu::*;
use crate::ui_window::*;
use crate::ui_menubar::*;
use crate::ui_popup::*;
use crate::game_app_event::*;
use crate::sb_state::*;
use crate::sb_cmdtype::*;
use crate::geometory::*;

const ZeroRect:SDL_Rect=SDL_Rect{x:0,y:0,w:0,h:0};
const BgImageRect:SDL_Rect=SDL_Rect{x:0,y:0,w:WND_W,h:WND_H};
const LeftBustupImageRect:SDL_Rect=SDL_Rect{x:0,y:0,w:WND_W/2,h:WND_H};
const RightBustupImageRect:SDL_Rect=SDL_Rect{x:WND_W/2,y:0,w:WND_W/2,h:WND_H};
const TextAreaRect:SDL_Rect=SDL_Rect{x:0,y:WND_H-MSG_TEXT_SIZE*6,w:WND_W,h:MSG_TEXT_SIZE*6};
///レンダリングモード
#[derive(Clone)]
pub enum RENDER_MODE{
    ///通常転送
    Norm,
    ///フェードイン
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Fadein(u32),
    ///フェードアウト
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Fadeout(u32),
    /// ブラックイン
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Blackin(u32),
    /// ブラックアウト
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Blackout(u32),
    /// 左からのムーブイン
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    MoveinFromLeft(u32),
    /// 右からのムーブイン
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    MoveinFromRight(u32),
    /// 左へのムーブアウト
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    MoveoutToLeft(u32),
    /// 右へのムーブアウト
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    MoveoutToRight(u32),
    /// シャッターイン
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Shutterin(u32),
    /// シャッターアウト
    ///* 'u32'  - 0〜DEF_FRAME_RATEまでの整数
    Shutterout(u32),
}
/// 選択肢の情報
pub struct SelectionInfo{
    ///* 'msg' 選択肢で」表示するメッセージ
    pub msg:String,
    ///* 'rect' 画面内の選択肢を表示する位置 
    pub rect:SDL_Rect,
}

/// ゲームアプリ管理オブジェクト
pub struct GameApp{
    ///* 'app' アプリオブジェクト
    pub app:App,
    ///* 'left_image' 左バストアップイメージ
    pub left_image:String,
    ///* 'show_left' 左バストアップの表示方法
    pub show_left:Option<RENDER_MODE>,
    ///* 'right_image' 右バストアップイメージ
    pub right_image:String,
    ///* 'show_right' 右バストアップの表示方法
    pub show_right:Option<RENDER_MODE>,
    ///* 'bg_image' 背景イメージ
    pub bg_image:String,
    ///* 'show_bg' 背景の表示方法
    pub show_bg:Option<RENDER_MODE>,
    pub player_battle_image:String,
    pub show_player_battle:Option<RENDER_MODE>,
    ///* 'text_log' 画面に表示されうテキストMSGのログ
    pub text_log:[String;TEXT_LOG_NUM],
    ///* 'text_log_pos' 画面に表示されているtext_log内のMSG
    pub text_log_pos:usize,
    ///* 'text_changed' 表示を行うテキストが変更されたならtrue
    pub text_changed:bool,

    ///* 'menu_bar' 画面上部に表示されているメニューバー
    menu_bar:Menubar,
    ///* 'image_origin_rect_tbl' 画面内の表示位置が決まっているイメージの位置座標
    image_origin_rect_tbl:[SDL_Rect;4],
    //
    blt_rect_tbl:[SDL_Rect;4],
    ///* 'script' 実行中のスクリプト
    script:Option<SB_State>,
    ///* 'selections' 画面に表示されている選択肢
    pub selections:Option<[SelectionInfo;4]>
}
impl GameApp{

    ///* コンストラクタ
    pub fn new()->Self{
        return Self{
            app:App::new(),
            left_image:String::new(),
            show_left:None,
            right_image:String::new(),
            show_right:None,
            bg_image:String::new(),
            show_bg:None,
            player_battle_image:String::new(),
            show_player_battle:None,
            text_log:[const {String::new()};TEXT_LOG_NUM],
            text_log_pos:0,
            text_changed:false,
            menu_bar:Menubar::new(

                
                vec![
                    MenuItem::new(MenuType::Submenu("File".to_string(),
                        Some(
                            PopupMenu::new(vec![MenuItem::new(MenuType::Text("Quit".to_string(),Some(quit_app)))])
                        ))),
                ]
            ),
            image_origin_rect_tbl:[
                BgImageRect,
                LeftBustupImageRect,
                RightBustupImageRect,
                TextAreaRect
            ],
            blt_rect_tbl:[ZeroRect;4],
            script:None,
            selections:None  
        };
    }
    ///* 初期化イベント処理
    pub fn on_init(&mut self){
        self.menu_bar.set_rect(&self.app,&SDL_Rect{x:0,y:0,w:640,h:480});

    }
    ///* ゲーム更新イベント処理
    pub fn update(&mut self)->bool{

        //スクリプトを実行しても良いか？
        let mut enable_update=true;
        //背景を更新
        if let Some(mode)=&self.show_bg{
            let src_rect=&self.image_origin_rect_tbl[0];
            let mut dst_rect=*src_rect;
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            self.blt_rect_tbl[0]=dst_rect;
            if let Some(RENDER_MODE::Norm)=next{
                enable_update=false;
            }
            self.show_bg=next;


        }
        if let Some(mode)=&self.show_left{
            let src_rect=SDL_Rect{x:0,y:0,w:WND_W/2,h:WND_H};
            let mut dst_rect=src_rect;
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            self.blt_rect_tbl[1]=dst_rect;
            if let Some(RENDER_MODE::Norm)=next{
                enable_update=false;
            }
            
            self.show_left=next;
        }
        if let Some(mode)=&self.show_right{
            let src_rect=SDL_Rect{x:0,y:0,w:WND_W/2,h:WND_H};
            let mut dst_rect=SDL_Rect{x:WND_W/2,y:0,w:WND_W/2,h:WND_H};
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            self.blt_rect_tbl[2]=dst_rect;
            if let Some(RENDER_MODE::Norm)=next{
                enable_update=false;
            }
            
            self.show_right=next;
            
        }
        if self.text_changed{
            let text_rect=self.image_origin_rect_tbl[3];
            self.app.set_gpage(TEXT_GPAGE,RENDER_GPAGE);
            self.app.set_draw_color(0x00,0x00,0x00,0xFF);
            if let Some(sel)=&self.selections{
                self.app.fill_rect(&SDL_Rect{x:0,y:0,w:WND_W,h:WND_H});
                self.app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
                for i in sel{
                    if 0<i.msg.len(){
                        self.app.draw_msg(i.rect.x,i.rect.y,i.msg.as_str());
                    }
                }
            }else{
                self.app.fill_rect(&text_rect);
                self.app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
                let text_idx=(self.text_log_pos+self.text_log.len()-1)%self.text_log.len();
                let text=&self.text_log[text_idx];
                if 0<text.len(){
                    self.app.draw_msg(text_rect.x,text_rect.y,text.as_str());

                }

            }
            self.text_changed=false;
        }
        /*if 0==self.menu_bar.rect.w{
            self.menu_bar.set_rect(&self.app,&SDL_Rect{x:0,y:0,w:640,h:480});
        }*/
        if self.app.click(){
            let ud=self as *mut _ as  *mut c_void;
            let click_pos=self.app.click_pos();
            if !self.menu_bar.click(&click_pos,&self.app,ud){
                let click_rect=SDL_Rect{
                    x:click_pos.x,y:click_pos.y,w:1,h:1};
                
                if let Some(sel)=&self.selections{
                    let mut click_sel=false;
                    for i in 0..sel.len(){
                        if 0<sel[i].msg.len(){
                            unsafe{
                                if SDL_bool_SDL_TRUE==
                                    SDL_HasIntersection(&sel[i].rect,&click_rect){
                                    if let Some(s)=&mut self.script{
                                        s.share_var_tbl.insert(
                                            String::from("result"),
                                            VarType::Int(i as i32));
                                        click_sel=true;
                                        self.selections=None;
                                        self.text_changed=true;
                                        break;
                                    }
                                }

                            }
                        }
                    }

                    enable_update=enable_update && click_sel;
                }
            }
        }
        if enable_update{
            if let Some(s)=&mut self.script{
                s.run_script=true;
                s.run();
            }
            
        }
        return enable_update;
    }
    pub fn paint(&mut self){
        self.app.set_gpage(0,0);
        if let Some(mode)=&self.show_bg{
            unsafe{
                if SDL_bool_SDL_TRUE==
                    SDL_HasIntersection(&self.image_origin_rect_tbl[0],
                        &self.app.dirty_rect_tbl[BG_GPAGE]){
                    let repaint_rect=self.app.dirty_rect_tbl[BG_GPAGE];
                    self.app.copy(BG_GPAGE,
                        &repaint_rect,
                        &repaint_rect);
                        
                }
            }
        }        
        if let Some(mode)=&self.show_left{
            unsafe{
                if SDL_bool_SDL_TRUE==
                    SDL_HasIntersection(&self.image_origin_rect_tbl[1],
                        &self.app.dirty_rect_tbl[BUSTUP_GPAGE]){
                    let mut repaint_rect=ZeroRect;
                    SDL_IntersectRect(&self.image_origin_rect_tbl[1],
                        &self.app.dirty_rect_tbl[BUSTUP_GPAGE],
                        &mut repaint_rect);
                    self.app.copy(BUSTUP_GPAGE,&repaint_rect,&repaint_rect);
                        
                }

            }
        }        
        if let Some(mode)=&self.show_right{
            unsafe{
                if SDL_bool_SDL_TRUE==
                    SDL_HasIntersection(&self.image_origin_rect_tbl[2],
                    &self.app.dirty_rect_tbl[BUSTUP_GPAGE]){
                    let mut repaint_rect=ZeroRect;
                    SDL_IntersectRect(&self.image_origin_rect_tbl[2],
                        &self.app.dirty_rect_tbl[BUSTUP_GPAGE],
                        &mut repaint_rect);
                    self.app.copy(BUSTUP_GPAGE,&repaint_rect,&repaint_rect);
                        
                }

            }
            
        }
        unsafe{
            if rect_has_intersect(&self.image_origin_rect_tbl[2],
                    &self.app.dirty_rect_tbl[TEXT_GPAGE]){

                    let repaint_rect=self.app.dirty_rect_tbl[TEXT_GPAGE];
                    self.app.copy(TEXT_GPAGE,
                        &repaint_rect,
                        &repaint_rect);

                }
        }
/*
        if let Some(sel)=&self.selections{
            for i in sel{
                if 0<i.msg.len(){
                    self.app.draw_msg(i.rect.x,i.rect.y,i.msg.as_str());
                }
            }
        }else if self.text_changed{
                let text_idx=(self.text_log_pos+self.text_log.len()-1)%self.text_log.len();
                let text=&self.text_log[text_idx];
                    //println!("{}",text);
                self.app.draw_msg(0,400,text.as_str());
                self.text_changed=false;
        }*/
        //self.app.set_draw_color(0x00,0x00,0x00,0xFF);
        //self.app.fill_rect(&SDL_Rect{x:0,y:0,w:WND_W,h:100});
        self.app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
        self.menu_bar.render(&mut self.app);

    }
    pub fn render(&mut self)->bool{
        let mut enable_update=true;
        self.app.set_gpage(0,0);
        
        if let Some(mode)=&self.show_bg{
            let src_rect=SDL_Rect{x:0,y:0,w:WND_W,h:WND_H};
            let mut dst_rect=src_rect;
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            if let None=self.show_bg{
                enable_update=false;
            }
            self.app.copy(BG_GPAGE,&src_rect,&dst_rect);
            self.show_bg=next;
        }
        if let Some(mode)=&self.show_left{
            let src_rect=SDL_Rect{x:0,y:0,w:WND_W/2,h:WND_H};
            let mut dst_rect=src_rect;
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            if let None=self.show_left{
                enable_update=false;
            }
            self.app.copy(BUSTUP_GPAGE,&src_rect,&dst_rect);
            self.show_left=next;
        }
        if let Some(mode)=&self.show_right{
            let src_rect=SDL_Rect{x:0,y:0,w:WND_W/2,h:WND_H};
            let mut dst_rect=SDL_Rect{x:WND_W/2,y:0,w:WND_W/2,h:WND_H};
            let next=self.proc_draw_settings(BG_GPAGE,mode.clone(),&mut dst_rect);
            if let None=self.show_right{
                enable_update=false;
            }
            self.app.copy(BUSTUP_GPAGE,&src_rect,&dst_rect);
            self.show_right=next;
            
        }
        if self.text_changed{
            let text_idx=(self.text_log_pos+self.text_log.len()-1)%self.text_log.len();
            let text=&self.text_log[text_idx];
                //println!("{}",text);
            self.app.draw_msg(0,400,text.as_str());
            self.text_changed=false;

        }
        if 0==self.menu_bar.rect.w{
            self.menu_bar.set_rect(&self.app,&SDL_Rect{x:0,y:0,w:640,h:480});
        }
        self.app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
        if self.app.click(){
            let ud=self as *mut _ as  *mut c_void;
            self.menu_bar.click(&self.app.click_pos(),&self.app,ud);
        }
        self.menu_bar.render(&mut self.app);
        return enable_update;
    }
    fn proc_draw_settings(&mut self,gpage:usize,mode:RENDER_MODE,dst_rect:&mut SDL_Rect)->Option<RENDER_MODE>{
            
            match mode{
                RENDER_MODE::Norm=>{
                    return Some(mode);
                },
                RENDER_MODE::Fadein(cur_count)=>{
                   let value=(255/DEF_FRAME_RATE*(DEF_FRAME_RATE-cur_count)) as u8;
                   self.app.set_mod_alpha(gpage,value); 
                   if DEF_FRAME_RATE== cur_count{
                        return Some(RENDER_MODE::Norm);
                   }else{
                       return Some(RENDER_MODE::Fadein(cur_count+1));
                   }
                },
                RENDER_MODE::Fadeout(cur_count)=>{
                   let value=(255/DEF_FRAME_RATE*(cur_count)) as u8;
                   self.app.set_mod_alpha(BG_GPAGE,value); 
                   if DEF_FRAME_RATE== cur_count{
                        return None;
                   }else{
                       return Some(RENDER_MODE::Fadeout(cur_count+1));
                   }

                },
                RENDER_MODE::Blackin(cur_count)=>{
                   let value=(255/DEF_FRAME_RATE*(cur_count)) as u8;
                   self.app.set_mod_color(BG_GPAGE,value,value,value);
                   if DEF_FRAME_RATE== cur_count{
                        return Some(RENDER_MODE::Norm);
                   }else{
                       return Some(RENDER_MODE::Blackin(cur_count+1));
                   }
                },
                RENDER_MODE::Blackout(cur_count)=>{
                   let value=(255/DEF_FRAME_RATE*(DEF_FRAME_RATE-cur_count)) as u8;
                   self.app.set_mod_color(BG_GPAGE,value,value,value);
                   if DEF_FRAME_RATE== cur_count{
                        return None;
                   }else{
                       return Some(RENDER_MODE::Blackout(cur_count+1));
                   }
                },

                RENDER_MODE::MoveinFromLeft(cur_count)=>{
                    let value=dst_rect.w/(DEF_FRAME_RATE as i32)*
                            (cur_count as i32);
                    dst_rect.x=dst_rect.x-value;
                   if DEF_FRAME_RATE== cur_count{
                        return Some(RENDER_MODE::Norm);
                   }else{
                       return Some(RENDER_MODE::MoveinFromLeft(cur_count+1));
                   }
                },
                RENDER_MODE::MoveinFromRight(cur_count)=>{
                    let value=dst_rect.w/(DEF_FRAME_RATE as i32)*
                            (cur_count as i32);
                    dst_rect.x=dst_rect.x+value;
                   if DEF_FRAME_RATE== cur_count{
                        return Some(RENDER_MODE::Norm);
                   }else{
                       return Some(RENDER_MODE::MoveinFromRight(cur_count+1));
                   }
                },
                RENDER_MODE::MoveoutToLeft(cur_count)=>{
                    let value=dst_rect.w/(DEF_FRAME_RATE as i32)*
                            (cur_count as i32);
                    dst_rect.x=dst_rect.x-value;
                   if DEF_FRAME_RATE== cur_count{
                        return None;
                   }else{
                       return Some(RENDER_MODE::MoveoutToLeft(cur_count+1));
                   }
                },
                RENDER_MODE::MoveoutToRight(cur_count)=>{
                    let value=dst_rect.w/(DEF_FRAME_RATE as i32)*
                            (cur_count as i32);
                    dst_rect.x=dst_rect.x+value;
                   if DEF_FRAME_RATE== cur_count{
                        return None;
                   }else{
                       return Some(RENDER_MODE::MoveoutToRight(cur_count+1));
                   }
                },
                RENDER_MODE::Shutterin(cur_count)=>{
                    let value=dst_rect.h/(DEF_FRAME_RATE as i32)*
                        (cur_count as i32);
                    dst_rect.h=value as i32;
                   if DEF_FRAME_RATE== cur_count{
                        return Some(RENDER_MODE::Norm);
                   }else{
                       return Some(RENDER_MODE::Shutterin(cur_count+1));
                   }
                },
                RENDER_MODE::Shutterout(cur_count)=>{
                    let value=dst_rect.w/(DEF_FRAME_RATE as i32)*
                        ((DEF_FRAME_RATE-cur_count) as i32);
                    dst_rect.h=value;
                   if DEF_FRAME_RATE== cur_count{
                        return None;
                   }else{
                       return Some(RENDER_MODE::Shutterout(cur_count+1));
                   }
                },
            }


    }

    pub fn load_script(&mut self,file_path:String,p_ud:*mut c_void)->Result<(),String>{
        let r=SB_State::load_from_file(&file_path);
        if let Ok(mut s)=r{
            s.p_user_data=p_ud;
            self.script=Some(s);
        }else if let Err(msg)=r{
            return Err(msg);
        }
        return Ok(());
    }
}