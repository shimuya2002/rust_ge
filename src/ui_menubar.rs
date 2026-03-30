use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::size::*;
use crate::config::*;
use crate::point::*;
use crate::ui_item::*;
use crate::ui_menu::*;
use crate::ui_popup::*;
///Menubar UI
pub struct Menubar{
    sub_menus:Vec<MenuItem>,//画面上部に表示される内容
    pub rect:SDL_Rect,//メニューバーの領域
}
impl Menubar{
    ///# 引数
    ///* 'items' - メニューバーに表示する項目
    pub fn new(items:Vec<MenuItem>)->Self{
        return Self{
            sub_menus:items,
            rect:SDL_Rect{x:0,y:0,w:0,h:0}
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&SDL_Rect){
        self.rect=*preffered_rect;
        let mut child_pref_rect=*preffered_rect;
        child_pref_rect.x=child_pref_rect.x+UI_BORDER_SIZE;
        child_pref_rect.w=child_pref_rect.w-UI_BORDER_SIZE;
        let mut max_h=0;
        for mut i in &mut self.sub_menus{
            let r=i.set_rect(app,&child_pref_rect);
            if (r.x+r.w)>(child_pref_rect.x+child_pref_rect.w){
                break;
            }
            child_pref_rect.x=child_pref_rect.x+r.w+UI_BORDER_SIZE;
            child_pref_rect.w=child_pref_rect.w-r.w-UI_BORDER_SIZE;
            max_h=max(max_h,r.h);

        }
        self.rect.h=max_h+8;

    }
    pub fn render(&self,app:&mut App){
        for i in &self.sub_menus{
            i.render(app);
            let r=&i.rect;
            if (r.x+r.w)>(self.rect.x+self.rect.w){
                break;
            }
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
                            if PointInRect(po,&i.rect){
                                ppm.close();
                                return true;
                            }else{
                                return ppm.click(po,app,ud);

                            }
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
