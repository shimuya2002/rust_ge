use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::geometory::*;
use crate::config::*;
include!("./geometory_inc.rs");

pub type EventAction=fn(p_user_data:*mut c_void);
pub type ItemEventAction=fn(p_user_data:*mut c_void,item:usize);
pub type TextEventAction=fn(p_user_data:*mut c_void,text:String);
pub struct UIItem{
    rect:RectType,
    ui_type:UIType,
}
pub enum UIType{
    Space,
    Text(String),
    InputText(String,Option<TextEventAction>),
    Button(String,Option<EventAction>),
    Checkbox(bool,String,Option<EventAction>),
    Radiobutton(usize,usize,String,Option<ItemEventAction>),
    List(Vec<String>,Option<ItemEventAction>),
    Row(Vec<UIItem>),
    Column(Vec<UIItem>)
}
impl UIItem{
    pub fn new(item:UIType)->Self{
        return Self{
            rect:rect_type![0,0,0,0],
            ui_type:item
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&RectType)->RectType{
        match &mut self.ui_type{
            UIType::Space=>{
# [cfg(feature="use_sdl3")]
                let r_w=if UI_TEXT_SIZE as f32>preffered_rect.w{
                            preffered_rect.w
                        }else{
                            UI_TEXT_SIZE as f32
                        };
# [cfg(feature="use_sdl2")]
                let r_w=if UI_TEXT_SIZE>preffered_rect.w{preffered_rect.w}else{UI_TEXT_SIZE};

# [cfg(feature="use_sdl3")]
                let r_h=if UI_TEXT_SIZE as f32>preffered_rect.h{
                            preffered_rect.h
                        }else{
                            UI_TEXT_SIZE as f32
                        };
# [cfg(feature="use_sdl2")]
                let r_h=if UI_TEXT_SIZE>preffered_rect.h{
                            preffered_rect.h
                        }else{
                            UI_TEXT_SIZE
                        };

                self.rect=rect_type!{
                    preffered_rect.x,
                    preffered_rect.y,
                    r_w,
                    r_h
                };
            },
            UIType::Text(s)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w,
                    t_size.h
                ];
            },
            UIType::InputText(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w+UI_BORDER_SIZE*2,
                    t_size.h+UI_BORDER_SIZE*2
                ];

            },
            UIType::Button(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w+UI_BORDER_SIZE*2,
                    t_size.h+UI_BORDER_SIZE*2
                ];

            },
            UIType::Checkbox(_,s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w+UI_TEXT_SIZE,
                    t_size.h
                ];

            },
            UIType::Radiobutton(_,_,s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=rect_type![
                    preffered_rect.x,
                    preffered_rect.y,
                    t_size.w+UI_TEXT_SIZE,
                    t_size.h
                ];

            },
            UIType::List(items,_)=>{
                let mut child_pref_rect=*preffered_rect;
                for mut i in items{
                    let r=app.measure_ui_utf8(i.as_str());
# [cfg(feature="use_sdl3")]
                    let tw=r.w as f32;
# [cfg(feature="use_sdl2")]
                    let tw=r.w;
# [cfg(feature="use_sdl3")]
                    let th=r.h as f32;
# [cfg(feature="use_sdl2")]
                    let th=r.h;
                    if (th)>(child_pref_rect.h){
                        break;
                    }
                    child_pref_rect.y=child_pref_rect.y+th;
                    child_pref_rect.h=child_pref_rect.h-th;

                }
                self.rect=RectType{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:preffered_rect.w,
                    h:child_pref_rect.h+child_pref_rect.y-preffered_rect.y
                };
            },
            UIType::Row(items)=>{
                let mut child_pref_rect=*preffered_rect;
                for mut i in items{
                    let r=i.set_rect(app,&child_pref_rect);
                    if (r.x+r.w)>(child_pref_rect.x+child_pref_rect.w){
                        break;
                    }
                    child_pref_rect.x=r.x+r.w;
                    child_pref_rect.w=child_pref_rect.w-r.w;

                }
                self.rect=RectType{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:child_pref_rect.w+child_pref_rect.x-preffered_rect.x,
                    h:preffered_rect.h
                };
            },
            UIType::Column(items)=>{
                let mut child_pref_rect=*preffered_rect;
                for mut i in items{
                    let r=i.set_rect(app,&child_pref_rect);
                    if (r.y+r.h)>(child_pref_rect.y+child_pref_rect.h){
                        break;
                    }
                    child_pref_rect.y=r.y+r.h;
                    child_pref_rect.h=child_pref_rect.h-r.h;

                }
                self.rect=RectType{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:preffered_rect.w,
                    h:child_pref_rect.h+child_pref_rect.y-preffered_rect.y
                };
            }
        }
        return self.rect;
    }
    pub fn render(&self,app:&mut App){
        match &self.ui_type{
            UIType::Space=>{},
            UIType::Text(s)=>{
# [cfg(feature="use_sdl3")]
                let tx=(self.rect.x as i32);
# [cfg(feature="use_sdl2")]
                let tx=self.rect.x;
# [cfg(feature="use_sdl3")]
                let ty=(self.rect.y as i32);
# [cfg(feature="use_sdl2")]
                let ty=self.rect.y;
                app.draw_ui_text(tx,ty,s.as_str());
            },
            UIType::InputText(s,_)=>{
                app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
                app.fill_rect(&self.rect);
                app.set_draw_color(0x00,0x00,0x00,0xFF);
# [cfg(feature="use_sdl3")]
                let tx=(self.rect.x as i32)+UI_BORDER_SIZE;
# [cfg(feature="use_sdl2")]
                let tx=self.rect.x+UI_BORDER_SIZE;
# [cfg(feature="use_sdl3")]
                let ty=(self.rect.y as i32)+UI_BORDER_SIZE;
# [cfg(feature="use_sdl2")]
                let ty=self.rect.y+UI_BORDER_SIZE;

                app.draw_ui_text(tx,ty,s.as_str());
                app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
            },
            UIType::Button(s,_)=>{
                app.draw_rect(&self.rect);

 # [cfg(feature="use_sdl3")]
                let tx=self.rect.x as i32+UI_BORDER_SIZE;
 # [cfg(feature="use_sdl2")]
                let tx=self.rect.x+UI_BORDER_SIZE;
 # [cfg(feature="use_sdl3")]
                let ty=self.rect.y as i32+UI_BORDER_SIZE;
 # [cfg(feature="use_sdl2")]
                let ty=self.rect.y+UI_BORDER_SIZE;
                app.draw_ui_text(tx,ty,s.as_str());

            },
            UIType::Checkbox(v,s,_)=>{
 # [cfg(feature="use_sdl3")]
                let cb_size=UI_TEXT_SIZE as f32;
 # [cfg(feature="use_sdl2")]
                let cb_size=UI_TEXT_SIZE;
                let check_rect=RectType{
                    x:self.rect.x,
                    y:self.rect.y,
                    w:cb_size,
                    h:cb_size};

                app.fill_rect(&check_rect);
                if *v{
                    app.set_draw_color(0x00,0x00,0x00,0xFF);
                    app.fill_rect(&check_rect);
                    app.set_draw_color(0xFF,0xFF,0xFF,0xFF);

                }
            },
            UIType::Radiobutton(iv,cv,s,_)=>{
                
 # [cfg(feature="use_sdl3")]
                    let rad_x=self.rect.x as i32+UI_TEXT_SIZE/2;
 # [cfg(feature="use_sdl2")]
                    let rad_x=self.rect.x+UI_TEXT_SIZE/2;
 # [cfg(feature="use_sdl3")]
                    let rad_y=self.rect.y as i32+UI_TEXT_SIZE/2;
 # [cfg(feature="use_sdl2")]
                    let rad_y=self.rect.y+UI_TEXT_SIZE/2;
                app.fill_circle(rad_x,rad_y,UI_TEXT_SIZE);
                if *iv==*cv{
                    app.set_draw_color(0x00,0x00,0x00,0xFF);

                    app.fill_circle(rad_x,rad_y,UI_TEXT_SIZE);
                    app.set_draw_color(0xFF,0xFF,0xFF,0xFF);

                }
                
            },
            UIType::List(items,_)=>{},
            UIType::Row(items)=>{
                for i in items{
                    i.render(app);
                }
            },
            UIType::Column(items)=>{
                for i in items{
                    i.render(app);
                }
            }
        }
    }
}
