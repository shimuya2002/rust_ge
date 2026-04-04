use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::config::*;
use crate::geometory::*;
use crate::ui_item::*;
include!("./geometory_inc.rs");

pub struct UIWindow{
    title:String,
    root:UIItem,
    rect:RectType,
    title_rect:RectType,
}
impl UIWindow{
    pub fn new(title:String,root:UIItem)->UIWindow{
        return Self{
            title:title,
            root:root,
            rect:gen_rect_i32(0,0,0,0),
            title_rect:gen_rect_i32(0,0,0,0)
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&RectType)->RectType{
        let t_size=app.measure_ui_utf8(self.title.as_str());
        self.title_rect=rect_type![
            preffered_rect.x,
            preffered_rect.y,
            t_size.w+UI_BORDER_SIZE*2,
            t_size.h+UI_BORDER_SIZE*2
        ];
# [cfg(feature="use_sdl3")]
        let ch=preffered_rect.h-(UI_BORDER_SIZE*2) as f32;
# [cfg(feature="use_sdl2")] 
        let ch=preffered_rect.h-(UI_BORDER_SIZE*2);

        let client_rect=self.root.set_rect(app,
            &rect_type!{
                preffered_rect.x,
                preffered_rect.y+self.title_rect.h,
                preffered_rect.w,
                ch});
        self.rect=rect_type!{
            self.title_rect.x,
            self.title_rect.y,
            if self.title_rect.w>client_rect.w{self.title_rect.w}else{client_rect.w},
            self.title_rect.h+client_rect.h
        };

        return self.rect;
    }
    pub fn render(&self,app:&mut App){
        app.draw_rect(&self.rect);
        app.draw_rect(&self.title_rect);
# [cfg(feature="use_sdl3")]
        let tx=(self.title_rect.x as i32)+UI_BORDER_SIZE;
# [cfg(feature="use_sdl2")] 
        let tx=self.title_rect.x+UI_BORDER_SIZE;

# [cfg(feature="use_sdl3")]
        let ty=(self.title_rect.y as i32)+UI_BORDER_SIZE;
# [cfg(feature="use_sdl2")]
        let ty=self.title_rect.y+UI_BORDER_SIZE;

        app.draw_ui_text(tx,ty,&self.title.as_str());
        self.root.render(app);
    }
}