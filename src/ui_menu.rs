use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::geometory::*;
use crate::config::*;
use crate::ui_item::*;
use crate::ui_popup::*;
include!("./geometory_inc.rs");

///* 画面内に表示されるメニュー項目
pub enum MenuType{
    ///テキスト
    ///* 'String' 表示される文字列
    ///* 'Option<EventAction>' クリックされた時に実行されるイベントハンドラー
    Text(String,Option<EventAction>),
    ///サブメニュー
    ///* 'String' 表示する文字列
    ///* 'Option<PopupMenu>' 選択された時に表示するサブメニュー
    Submenu(String,Option<PopupMenu>)
}
pub struct MenuItem{
    pub menu_type:MenuType,
    pub rect:RectType,
}
impl MenuItem{
    pub fn new(menu_type:MenuType)->Self{
        return Self{
            menu_type:menu_type,
            rect:rect_type!{0,0,0,0}
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&RectType)->RectType{
        match &mut self.menu_type{

            MenuType::Text(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w,
                    t_size.h
                ];

            },
            MenuType::Submenu(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w,
                    t_size.h
                ];

            }
        }
        return self.rect;
    }
    pub fn render(&self,app:&mut App){
        match &self.menu_type{
            MenuType::Text(s,_)=>{
# [cfg(feature="use_sdl3")]
                let tx=self.rect.x as i32;
# [cfg(feature="use_sdl2")]
                let tx=self.rect.x;
# [cfg(feature="use_sdl3")]
                let ty=self.rect.y as i32;
# [cfg(feature="use_sdl2")]
                let ty=self.rect.y;
                app.draw_ui_text(tx,ty,s.as_str());
            }
            MenuType::Submenu(s,popup)=>{
# [cfg(feature="use_sdl3")]
                let tx=self.rect.x as i32;
# [cfg(feature="use_sdl2")]
                let tx=self.rect.x;
# [cfg(feature="use_sdl3")]
                let ty=self.rect.y as i32;
# [cfg(feature="use_sdl2")]
                let ty=self.rect.y;


                app.draw_ui_text(tx,ty,s.as_str());
                if let Some(pp)=popup{
                    if pp.is_open{
                        pp.render(app);
                    }
                }
            }
        }
    }
    
}
