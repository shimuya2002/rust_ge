use std::fmt;

use crate::imports::*;
use crate::geometory::*;
#[derive(Clone)]
pub enum GPageRenderMode{
    None,
    Norm,
}
#[derive(Clone)]
pub struct Sprite{
    pub gpage:usize,
    pub src_rect:RectType,
    pub render_mode:GPageRenderMode,
}

impl fmt::Display for Sprite{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"gpage={},src_rect=[{},{},{},{}]",
            self.gpage,
            self.src_rect.x,self.src_rect.y,self.src_rect.w,self.src_rect.h)
    }
}