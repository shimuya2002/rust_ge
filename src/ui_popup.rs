use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::geometory::*;
use crate::config::*;
use crate::ui_item::*;
use crate::ui_menu::*;
include!("./geometory_inc.rs");

pub struct PopupMenu{
    sub_menus:Vec<MenuItem>,
    pub rect:RectType,
    pub is_open:bool

}
impl PopupMenu{
    pub fn new(items:Vec<MenuItem>)->Self{
        return Self{
            sub_menus:items,
            rect:rect_type!{0,0,0,0},
            is_open:false
        };
    }
    pub fn open(&mut self,app:&App,rect:&RectType){
        self.is_open=true;
        let mut preffered_rect=rect_type!{
            (rect.x as i32)+UI_BORDER_SIZE,
            (rect.y as i32)+UI_BORDER_SIZE,
            (rect.w as i32)-UI_BORDER_SIZE*2,
            (rect.y as i32)-UI_BORDER_SIZE*2
        };

        let mut max_h=0;
        let mut max_w=0;
        for i in &mut self.sub_menus{
            let tmp=i.set_rect(app,&preffered_rect);
            preffered_rect.y=(tmp.y+tmp.h);
            max_h=((tmp.y+tmp.h)-(rect.y)) as i32;
            max_w=max(tmp.w as i32,max_w as i32);
        }
# [cfg(feature="use_sdl3")]
        let r_rect=RectType{
            x:rect.x,
            y:rect.y,
            w:(max_w+16) as f32,
            h:(max_h+16) as f32
        };
# [cfg(feature="use_sdl2")]
        let r_rect=gen_rect_i32(rect.x,rect.y,max_w+16,max_h+16);
        self.rect=r_rect;
    }
    pub fn close(&mut self){
        self.is_open=false;
    }
    pub fn render(&self,app:&mut App){
        for i in &self.sub_menus{
            i.render(app);
        }
        app.draw_rect(&self.rect);
    }
    pub fn click(&mut self,po:&SDL_Point,app:&App,ud:*mut c_void)->bool{
        for i in &mut self.sub_menus{
            match &mut i.menu_type{
                &mut MenuType::Text(_,f)=>{
                    if PointInRect(po,&i.rect){
                        if let Some(func)=f{
                            func(ud);
                        }
                        self.close();
                        return true;
                    }
                },
                &mut MenuType::Submenu(_,ref mut popup)=>{
                    if let &mut Some(ref mut ppm)=popup{
                        if ppm.is_open{
                            return ppm.click(po,app,ud);
                        }else{
                            if PointInRect(po,&i.rect){
 # [cfg(feature="use_sdl3")]
                                let tmp_w=WND_W as f32-i.rect.x;
 # [cfg(feature="use_sdl2")]
                                let tmp_w=WND_W-i.rect.x;
 # [cfg(feature="use_sdl3")]
                                let tmp_h=WND_H as f32-i.rect.y;
 # [cfg(feature="use_sdl2")]
                                let tmp_h=WND_H-i.rect.y;
                                let tmp=RectType{
                                    x:i.rect.x,
                                    y:i.rect.y+i.rect.h,
                                    w:tmp_w,
                                    h:tmp_h
                                };
                                ppm.open(app,&tmp);
                                return true;
                            }

                        }
                    }   
                }
            
            }
                    
                
        }
        return false;
    }
}
