use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::config::*;
use crate::geometory::*;
use crate::ui_item::*;
use crate::ui_menu::*;
use crate::ui_popup::*;
///Menubar UI
pub struct Menubar{
    sub_menus:Vec<MenuItem>,//画面上部に表示される内容
    pub rect:RectType,//メニューバーの領域
}
impl Menubar{
    ///# 引数
    ///* 'items' - メニューバーに表示する項目
    pub fn new(items:Vec<MenuItem>)->Self{
        return Self{
            sub_menus:items,
            rect:gen_rect_i32(0,0,0,0)
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&RectType){
        self.rect=*preffered_rect;
        let mut child_pref_rect=*preffered_rect;
        {

# [cfg(feature="use_sdl3")]
            let ret_x=child_pref_rect.x+UI_BORDER_SIZE as f32;
# [cfg(feature="use_sdl2")]
            let ret_x=child_pref_rect.x+UI_BORDER_SIZE;
            child_pref_rect.x=ret_x;
# [cfg(feature="use_sdl3")]
            let ret_w=child_pref_rect.w-UI_BORDER_SIZE as f32;
# [cfg(feature="use_sdl2")]
            let ret_w=child_pref_rect.w-UI_BORDER_SIZE;
            child_pref_rect.w=ret_w;
        }
        let mut max_h=0;
        for mut i in &mut self.sub_menus{
            let r=i.set_rect(app,&child_pref_rect);
            if (r.x+r.w)>(child_pref_rect.x+child_pref_rect.w){
                break;
            }
# [cfg(feature="use_sdl3")]
            let ret_x=child_pref_rect.x+r.w+(UI_BORDER_SIZE as f32);
# [cfg(feature="use_sdl2")]
            let ret_x=child_pref_rect.x+r.w+UI_BORDER_SIZE;

            child_pref_rect.x=ret_x;
# [cfg(feature="use_sdl3")]
            let ret_w=child_pref_rect.w-r.w-(UI_BORDER_SIZE as f32);
# [cfg(feature="use_sdl2")]
            let ret_w=child_pref_rect.w-r.w-UI_BORDER_SIZE;
            child_pref_rect.w=ret_w;
            max_h=max(max_h,r.h as i32);

        }
# [cfg(feature="use_sdl3")]
        let ret_h=(max_h+8) as f32;
# [cfg(feature="use_sdl2")]
        let ret_h=max_h+8;
        
        self.rect.h=ret_h;

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
# [cfg(feature="use_sdl3")]
                                let tmp=RectType{
                                    x:i.rect.x,
                                    y:i.rect.y+i.rect.h,
                                    w:(WND_W as f32) -i.rect.x,
                                    h:(WND_H as f32)-i.rect.y
                                };
# [cfg(feature="use_sdl2")]
                                let tmp=gen_rect_i32(i.rect.x,
                                            i.rect.y+i.rect.h,
                                            WND_W -i.rect.x,
                                            WND_H-i.rect.y);
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
