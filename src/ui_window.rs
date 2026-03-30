use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::size::*;
use crate::config::*;
use crate::point::*;
use crate::ui_item::*;
pub struct UIWindow{
    title:String,
    root:UIItem,
    rect:SDL_Rect,
    title_rect:SDL_Rect,
}
impl UIWindow{
    pub fn new(title:String,root:UIItem)->UIWindow{
        return Self{
            title:title,
            root:root,
            rect:SDL_Rect{x:0,y:0,w:0,h:0},
            title_rect:SDL_Rect{x:0,y:0,w:0,h:0}
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&SDL_Rect)->SDL_Rect{
        let t_size=app.measure_ui_utf8(self.title.as_str());
        self.title_rect=SDL_Rect{
            x:preffered_rect.x,
            y:preffered_rect.y,
            w:t_size.w+UI_BORDER_SIZE*2,
            h:t_size.h+UI_BORDER_SIZE*2
        };

        let client_rect=self.root.set_rect(app,
            &SDL_Rect{
                x:preffered_rect.x,
                y:preffered_rect.y+self.title_rect.h,
                w:preffered_rect.w,
                h:preffered_rect.h-UI_BORDER_SIZE*2});
        self.rect=SDL_Rect{
            x:self.title_rect.x,
            y:self.title_rect.y,
            w:max(self.title_rect.w,client_rect.w),
            h:self.title_rect.h+client_rect.h
        };

        return self.rect;
    }
    pub fn render(&self,app:&mut App){
        app.draw_rect(&self.rect);
        app.draw_rect(&self.title_rect);
        app.draw_ui_text(self.title_rect.x+UI_BORDER_SIZE,
            self.title_rect.y+UI_BORDER_SIZE,
            &self.title.as_str());
        self.root.render(app);
    }
}