use std::collections::*;
use std::rc::*;
use crate::config::*;
use crate::imports::*;

///キャッシュしているアイテムの情報
struct CacheInfo<T>{
    ///前回参照した時刻
 # [cfg(feature="use_sdl3")]
    tick:u64,
 # [cfg(feature="use_sdl2")]
    tick:u32,
    ///キャッシュしているオブジェクト
    obj:Rc<T>
}
///キャッシュテーブル
pub struct CacheTbl<T>{
    tbl:HashMap<String,CacheInfo<T>>
}

impl<T:Drop> CacheTbl<T>{
    ///コンストラクタ
    pub fn new()->Self{
        return Self{
            tbl:HashMap::new()
        };
    }
    ///キャッシュテーブルに追加する
    /// 'key' 追加するアイテムを検索するときに使用するキー値
    /// 'obj' 追加するアイテム
    pub fn insert(&mut self,key:&String,obj:Rc<T>){
        if CACHE_MAX_NUM==self.tbl.len(){
            self.clear_old();
        }
        unsafe{
            self.tbl.insert(key.clone(),CacheInfo{tick:SDL_GetTicks(),obj:obj});

        }
    }
    /// キャッシュされているアイテムの検索
    /// 'key' アイテムの検索に使用するキー値
    /// キャッシュ内に存在しなければNoneを返す
    pub fn get(&mut self,key:&String)->Option<Rc<T>>{
        if self.tbl.contains_key(key){
            unsafe{
                self.tbl.get_mut(key).unwrap().tick=SDL_GetTicks();
                return Some(self.tbl[key].obj.clone());
            }
        }
        return None;
    }
    ///一番昔にアクセスされたアイテムをキャッシュから削除する
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
    ///保持しているアイテムをすべて消去する
    pub fn clear(&mut self){
        self.tbl.clear();
    }
}