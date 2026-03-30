use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::size::*;
use crate::config::*;
use crate::point::*;
use crate::ui_item::*;
use crate::ui_menu::*;

pub struct PopupMenu{
    sub_menus:Vec<MenuItem>,
    pub rect:SDL_Rect,
    pub is_open:bool

}
impl PopupMenu{
    pub fn new(items:Vec<MenuItem>)->Self{
        return Self{
            sub_menus:items,
            rect:SDL_Rect{x:0,y:0,w:0,h:0},
            is_open:false
        };
    }
    pub fn open(&mut self,app:&App,rect:&SDL_Rect){
        self.is_open=true;
        let mut preffered_rect=SDL_Rect{
            x:rect.x+UI_BORDER_SIZE,
            y:rect.y+UI_BORDER_SIZE,
            w:rect.w-UI_BORDER_SIZE*2,
            h:rect.y-UI_BORDER_SIZE*2
        };

        let mut max_h=0;
        let mut max_w=0;
        for i in &mut self.sub_menus{
            let tmp=i.set_rect(app,&preffered_rect);
            preffered_rect.y=tmp.y+tmp.h;
            max_h=(tmp.y+tmp.h)-rect.y;
            max_w=max(tmp.w,max_w);
        }
        self.rect=SDL_Rect{
            x:rect.x,
            y:rect.y,
            w:max_w+16,
            h:max_h+16
        };
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
                        return true;
                    }
                },
                &mut MenuType::Submenu(_,ref mut popup)=>{
                    if let &mut Some(ref mut ppm)=popup{
                        if ppm.is_open{
                            return ppm.click(po,app,ud);
                        }else{
                            if PointInRect(po,&i.rect){
                                let tmp=SDL_Rect{
                                    x:i.rect.x,
                                    y:i.rect.y+i.rect.h,
                                    w:WND_W-i.rect.x,
                                    h:WND_H-i.rect.y
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
