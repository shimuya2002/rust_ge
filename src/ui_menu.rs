use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::size::*;
use crate::config::*;
use crate::point::*;
use crate::ui_item::*;
use crate::ui_popup::*;
pub enum MenuType{
    Text(String,Option<EventAction>),
    Submenu(String,Option<PopupMenu>)
}
pub struct MenuItem{
    pub menu_type:MenuType,
    pub rect:SDL_Rect,
}
impl MenuItem{
    pub fn new(menu_type:MenuType)->Self{
        return Self{
            menu_type:menu_type,
            rect:SDL_Rect{x:0,y:0,w:0,h:0}
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&SDL_Rect)->SDL_Rect{
        match &mut self.menu_type{

            MenuType::Text(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w,
                    h:t_size.h
                };

            },
            MenuType::Submenu(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w,
                    h:t_size.h
                };

            }
        }
        return self.rect;
    }
    pub fn render(&self,app:&mut App){
        match &self.menu_type{
            MenuType::Text(s,_)=>{
                app.draw_ui_text(self.rect.x,self.rect.y,s.as_str());
            }
            MenuType::Submenu(s,popup)=>{
                app.draw_ui_text(self.rect.x,self.rect.y,s.as_str());
                if let Some(pp)=popup{
                    if pp.is_open{
                        pp.render(app);
                    }
                }
            }
        }
    }
    
}
