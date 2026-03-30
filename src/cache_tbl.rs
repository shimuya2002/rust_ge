use std::collections::*;
use std::rc::*;
use crate::config::*;
use crate::imports::*;
struct CacheInfo<T>{
 # [cfg(feature="use_sdl3")]
    tick:u64,
 # [cfg(feature="use_sdl2")]
    tick:u32,
    obj:Rc<T>
}
pub struct CacheTbl<T>{
    tbl:HashMap<String,CacheInfo<T>>
}

impl<T:Drop> CacheTbl<T>{
    pub fn new()->Self{
        return Self{
            tbl:HashMap::new()
        };
    }
    pub fn insert(&mut self,key:&String,obj:Rc<T>){
        if CACHE_MAX_NUM==self.tbl.len(){
            self.clear_old();
        }
        unsafe{
            self.tbl.insert(key.clone(),CacheInfo{tick:SDL_GetTicks(),obj:obj});

        }
    }
    pub fn get(&mut self,key:&String)->Option<Rc<T>>{
        if self.tbl.contains_key(key){
            unsafe{
                self.tbl.get_mut(key).unwrap().tick=SDL_GetTicks();
                return Some(self.tbl[key].obj.clone());
            }
        }
        return None;
    }
    fn clear_old(&mut self){
        let mut old_key=None;
        for i in self.tbl.keys(){
            if let None=old_key{
                old_key=Some(i.clone());
            }else if let Some(k)=&old_key{
                if self.tbl[&k.clone()].tick>
                    self.tbl[i].tick{
                    old_key=Some(i.clone());
                }

            
            }
        }
        if let Some(i)=old_key{
            self.tbl.remove(&i.clone());
        }
    }
    pub fn clear(&mut self){
        self.tbl.clear();
    }
}