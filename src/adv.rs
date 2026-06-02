use std::rc::*;

use crate::imports::*;
use crate::geometory::*;
use crate::app::*;
use crate::sprite::*;
pub struct Adv{
    ///画面に表示するメッセージが更新されたか？
    msg_change:bool,

    request_state_change:bool,
    ///メッセージ
    msg:Option<String>,

    bg_sprite:Option<Rc<Sprite>>,
    left_bustup_sprite:Option<Rc<Sprite>>,
    right_bustup_sprite:Option<Rc<Sprite>>,
    text_sprite:Option<Rc<Sprite>>,

}
impl Adv{
    pub fn new()->Self{
        return Adv{
            msg_change:false,

            request_state_change:true,
            msg:None,

            bg_sprite:None,
            left_bustup_sprite:None,
            right_bustup_sprite:None,
            text_sprite:None,
        };
    }

    pub fn set_bg_image(&mut self,sprite:Option<Rc<Sprite>>){
        self.bg_sprite=sprite;
    }
    pub fn set_left_bustup_image(&mut self,sprite:Option<Rc<Sprite>>){
        self.left_bustup_sprite=sprite;
    }
    pub fn set_right_bustup_image(&mut self,sprite:Option<Rc<Sprite>>){
        self.right_bustup_sprite=sprite;
    }    
    pub fn update_finished(&self)->bool{
        return self.request_state_change;
    }
    pub fn set_msg(&mut self,txt:String){
        self.msg=Some(txt);
        self.msg_change=true;
    }
    pub fn set_msg_rect(&mut self,sprite:Option<Rc<Sprite>>){
        self.text_sprite=sprite;
    }
    pub fn update(&mut self,app:&mut App){
        if self.msg_change{

            if let Some(txt)=&self.msg{
                if let Some(sprite)=&self.text_sprite{
                    app.set_gpage(sprite.gpage,0);
 # [cfg(feature="use_sdl3")]
                    let x=sprite.src_rect.x as i32;
 # [cfg(feature="use_sdl2")]                
                    let x=sprite.src_rect.x;

# [cfg(feature="use_sdl3")]
                    let y=sprite.src_rect.y as i32;
 # [cfg(feature="use_sdl2")]                
                    let y=sprite.src_rect.y;
                    app.set_draw_color(0xFF,0xFF,0xFF,0xFF);
                    app.draw_rect(&sprite.src_rect);
                    app.draw_msg(x,y,txt);
                    self.msg_change=false;
                }

            }
            
        }
    }
    pub fn render(&mut self,app:&mut App){

        app.set_gpage(0,0);
 # [cfg(feature="use_sdl3")]
        let cmp_value=0.0;
 # [cfg(feature="use_sdl2")]                
        let cmp_value=0;

        if let Some(txt)=&self.msg{
            if let Some(sprite)=&self.text_sprite{
                if cmp_value < app.dirty_rect_tbl[sprite.gpage].w{

                    app.copy(sprite.gpage,&sprite.src_rect,&sprite.src_rect);
                }
            }

        }
    }
    pub fn proc_user_click(&mut self,pos:&SDL_Point){
        self.request_state_change=true;
    }
}