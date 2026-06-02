use std::rc::*;
use crate::imports::*;
use crate::geometory::*;
use crate::sprite::*;


pub struct Animation{
    pub frames:Vec<Rc<Sprite>>,

}
