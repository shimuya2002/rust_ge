use std::rc::*;
use crate::imports::*;
use crate::geometory::*;
use crate::texture::*;

pub struct AnimFrame{
    pub rect:SDL_Rect,
    pub frame:i32
}
impl Default for AnimFrame{
    fn default()->Self{
        return Self{
            rect:SDL_Rect{x:0,y:0,w:0,h:0},
            frame:0
        };
    }
}
pub struct AnimSet{
    pub name:String,
    pub sprite:Option<usize>,
    origin_rect:SDL_Rect,
    offset_size:Size,
    frame_num:i32,
    pub current_frame:AnimFrame
}
impl AnimSet{
    pub fn new(name:String,sprite:usize,origin:&SDL_Rect,offset:&Size,num:i32)->Self{
        assert!(0<num);
        return Self{
            name:name,
            sprite:Some(sprite),
            origin_rect:*origin,
            offset_size:offset.clone(),
            frame_num:num,
            current_frame:AnimFrame{
                rect:*origin,
                frame:0
            }
        };
    }
    pub fn next(&mut self)->&AnimFrame{
        if (self.frame_num-1)>self.current_frame.frame{
            self.current_frame.rect.x=self.current_frame.rect.x+self.offset_size.w;
            self.current_frame.rect.y=self.current_frame.rect.y+self.offset_size.h;
        }else{
            self.current_frame.rect=self.origin_rect;
            self.current_frame.frame=0;
        }
        return &self.current_frame;
    }
}
impl Default for AnimSet{
    fn default()->Self{
        return Self{
            name:String::new(),
            sprite:None,
            origin_rect:SDL_Rect{x:0,y:0,w:0,h:0},
            offset_size:Size{w:0,h:0},
            frame_num:0,
            current_frame:AnimFrame::default(),
        }
    }
}