# [cfg(feature="use_sdl3")]
macro_rules! rect_type{
    ($x:expr,$y:expr,$w:expr,$h:expr)=>{
            SDL_FRect{
            x:$x as f32,
            y:$y as f32,
            w:$w as f32,
            h:$h as f32
        }
    };
}
# [cfg(feature="use_sdl2")]
macro_rules! rect_type{
    ($x:expr,$y:expr,$w:expr,$h:expr)=>{
        SDL_Rect{
            x:$x as i32,
            y:$y as i32,
            w:$w as i32,
            h:$h as i32
        }
    };
}
