use std::ffi::*;
use std::cmp::*;
use crate::imports::*;
use crate::app::*;
use crate::size::*;
use crate::config::*;
use crate::point::*;

pub type EventAction=fn(p_user_data:*mut c_void);
pub type ItemEventAction=fn(p_user_data:*mut c_void,item:usize);
pub type TextEventAction=fn(p_user_data:*mut c_void,text:String);
pub struct UIItem{
    rect:SDL_Rect,
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
            rect:SDL_Rect{x:0,y:0,w:0,h:0},
            ui_type:item
        };
    }
    pub fn set_rect(&mut self,app:&App,preffered_rect:&SDL_Rect)->SDL_Rect{
        match &mut self.ui_type{
            UIType::Space=>{
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:if UI_TEXT_SIZE>preffered_rect.w{preffered_rect.w}else{UI_TEXT_SIZE},
                    h:if UI_TEXT_SIZE>preffered_rect.h{preffered_rect.h}else{UI_TEXT_SIZE},
                };
            },
            UIType::Text(s)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w,
                    h:t_size.h
                };
            },
            UIType::InputText(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w+UI_BORDER_SIZE*2,
                    h:t_size.h+UI_BORDER_SIZE*2
                };

            },
            UIType::Button(s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w+UI_BORDER_SIZE*2,
                    h:t_size.h+UI_BORDER_SIZE*2
                };

            },
            UIType::Checkbox(_,s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w+UI_TEXT_SIZE,
                    h:t_size.h
                };

            },
            UIType::Radiobutton(_,_,s,_)=>{
                let t_size=app.measure_ui_utf8(s.as_str());
                self.rect=SDL_Rect{
                    x:preffered_rect.x,
                    y:preffered_rect.y,
                    w:t_size.w+UI_TEXT_SIZE,
                    h:t_size.h
                };

            },
            UIType::List(items,_)=>{
                let mut child_pref_rect=*preffered_rect;
                for mut i in items{
                    let r=app.measure_ui_utf8(i.as_str());
                    if (r.h)>(child_pref_rect.h){
                        break;
                    }
                    child_pref_rect.y=child_pref_rect.y+r.h;
                    child_pref_rect.h=child_pref_rect.h-r.h;

                }
                self.rect=SDL_Rect{
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
                self.rect=SDL_Rect{
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
                self.rect=SDL_Rect{
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
                app.draw_ui_text(self.rect.x,self.rect.y,s.as_str());
            },
            UIType::InputText(s,_)=>{
                app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
                app.fill_rect(&self.rect);
                app.set_draw_color(0x00,0x00,0x00,0xFF);
                app.draw_ui_text(self.rect.x+UI_BORDER_SIZE,self.rect.y+UI_BORDER_SIZE,s.as_str());
                app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
            },
            UIType::Button(s,_)=>{
                app.draw_rect(&self.rect);
                app.draw_ui_text(self.rect.x+UI_BORDER_SIZE,self.rect.y+UI_BORDER_SIZE,s.as_str());

            },
            UIType::Checkbox(v,s,_)=>{
                let check_rect=SDL_Rect{
                    x:self.rect.x,
                    y:self.rect.y,
                    w:UI_TEXT_SIZE,
                    h:UI_TEXT_SIZE};

                app.fill_rect(&check_rect);
                if *v{
                    app.set_draw_color(0x00,0x00,0x00,0xFF);
                    app.fill_rect(&check_rect);
                    app.set_draw_color(0xFF,0xFF,0xFF,0xFF);

                }
            },
            UIType::Radiobutton(iv,cv,s,_)=>{
                
                app.fill_circle(
                    self.rect.x+UI_TEXT_SIZE/2,
                    self.rect.y+UI_TEXT_SIZE/2,
                    UI_TEXT_SIZE);
                if *iv==*cv{
                    app.set_draw_color(0x00,0x00,0x00,0xFF);
                    app.fill_circle(
                        self.rect.x+UI_TEXT_SIZE/2,
                        self.rect.y+UI_TEXT_SIZE/2,
                        UI_TEXT_SIZE);
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
