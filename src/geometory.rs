use crate::imports::*;

pub fn rect_has_intersect(r1:&SDL_Rect,r2:&SDL_Rect)->bool{
    unsafe{
 # [cfg(feature="use_sdl3")]
        let r=SDL_HasRectIntersection(r1,r2);
 # [cfg(feature="use_sdl2")]
        let r=SDL_HasIntersection(r1,r2);
        return SDL_bool_SDL_TRUE==r;
    }
    
}
#[derive(Clone)]
pub struct Size{
    pub w:i32,
    pub h:i32
}
