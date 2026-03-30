use crate::imports::*;
pub fn PointInRect(po:&SDL_Point,rect:&SDL_Rect)->bool{
    return (po.x > rect.x) && 
           (po.x<(rect.x+rect.w)) &&
           (po.y > rect.y) &&
           (po.y < (rect.y+rect.h));
}