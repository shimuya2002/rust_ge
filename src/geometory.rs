
use crate::imports::*;
# [cfg(feature="use_sdl3")]
pub type RectType=SDL_FRect;
# [cfg(feature="use_sdl2")]
pub type RectType=SDL_Rect;


pub fn gen_rect_i32(x:i32,y:i32,w:i32,h:i32)->RectType{
# [cfg(feature="use_sdl3")]
    return SDL_FRect{
        x:x as f32,
        y:y as f32,
        w:w as f32,
        h:h as f32
    };
# [cfg(feature="use_sdl2")]
    return SDL_Rect{
        x:x as i32,
        y:y as i32,
        w:w as i32,
        h:h as i32
    };
}
pub fn rect_has_intersect(r1:&RectType,r2:&RectType)->bool{
    unsafe{
 # [cfg(feature="use_sdl3")]
        return SDL_HasRectIntersectionFloat(r1,r2);
 # [cfg(feature="use_sdl2")]
        let r=SDL_HasIntersection(r1,r2);

 # [cfg(feature="use_sdl2")]
        return SDL_bool_SDL_TRUE==r;
    }
    
}
pub fn rect_get_intersection(r1:&RectType,r2:&RectType,out:&mut RectType){
    unsafe{
 # [cfg(feature="use_sdl3")]
        SDL_GetRectIntersectionFloat(r1,r2,out);
# [cfg(feature="use_sdl2")]
        SDL_IntersectRect(r1,r2,out);

    }
}
pub fn rect_get_union(r1:&RectType,r2:&RectType,out:&mut RectType){
    unsafe{
# [cfg(feature="use_sdl3")]
        SDL_GetRectUnionFloat(r1,r2,out);
# [cfg(feature="use_sdl2")]
        SDL_UnionRect(r1,r2,out);
    }
}
#[derive(Clone)]
pub struct Size{
    pub w:i32,
    pub h:i32
}
pub fn PointInRect(po:&SDL_Point,rect:&RectType)->bool{
# [cfg(feature="use_sdl3")]
    let inc_x1=(po.x as f32) > rect.x;
# [cfg(feature="use_sdl2")]
    let inc_x1=po.x > rect.x;

# [cfg(feature="use_sdl3")]
    let inc_x2=(po.x as f32)<(rect.x+rect.w);
# [cfg(feature="use_sdl2")]
    let inc_x2=po.x<(rect.x+rect.w);

# [cfg(feature="use_sdl3")]
    let inc_y1=(po.y as f32)> rect.y;
# [cfg(feature="use_sdl2")]
    let inc_y1=po.y > rect.y;


# [cfg(feature="use_sdl3")]
    let inc_y2=(po.y as f32)< (rect.y+rect.h);
# [cfg(feature="use_sdl2")]
    let inc_y2=po.y < (rect.y+rect.h);
    return inc_x1 && 
           inc_x2 &&
           inc_y1 &&
           inc_y2;
}


# [cfg(feature="use_sdl3")]
pub const ZeroRect:SDL_FRect=SDL_FRect{x:0.0,y:0.0,w:0.0,h:0.0};
# [cfg(feature="use_sdl2")]
pub const ZeroRect:SDL_Rect=SDL_Rect{x:0,y:0,w:0,h:0};
