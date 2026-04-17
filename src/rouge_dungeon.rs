use std::cmp::*;

use crate::imports::*;
use crate::config::*;
use crate::geometory::*;
use crate::app::*;

include!("./geometory_inc.rs");
/// ローグ型ダンジョン
pub struct RougeDungon{
    rooms:[RectType;ROUGE_ROOM_MAX],
    room_num:usize,
    render_rect:RectType,
    floor:[u8;(ROUGE_ROOM_MAX_W*ROUGE_ROOM_MAX_H) as usize]
}

impl RougeDungon{
    ///コンストラクタ
    pub fn new()->Self{
        return Self{
            rooms:[ZeroRect;ROUGE_ROOM_MAX],
            room_num:0,
            render_rect:ZeroRect,
            floor:[0;(ROUGE_ROOM_MAX_W*ROUGE_ROOM_MAX_H) as usize]
        };
    }
    pub fn set_render_rect(&mut self,rect:&RectType){
        self.render_rect=*rect;
    }
    pub fn render(&self,app:&mut App){
        app.set_draw_color(255,255,255,255);
        let block_w=(self.render_rect.w as i32)/ROUGE_ROOM_MAX_W;
        let block_h=(self.render_rect.h as i32)/ROUGE_ROOM_MAX_H;
        for i in 0..ROUGE_ROOM_MAX_H{
            for j in 0..ROUGE_ROOM_MAX_W{
                if 0!=self.floor[(i*(ROUGE_ROOM_MAX_W as i32)+j) as usize]{
                    app.draw_rect(&rect_type!{
                        (self.render_rect.x as i32)+j*block_w,
                        (self.render_rect.y as i32)+i*block_h,
                        block_w,
                        block_h});
                }
            }
        }
    }
    pub fn reset(&mut self){
        self.room_num=0;
        for i in 0..self.rooms.len(){
            self.rooms[i]=ZeroRect;
        }
        for i in 0..self.floor.len(){
            self.floor[i]=0;
        }
        self.create_room(&rect_type!{0,0,ROUGE_ROOM_MAX_W,ROUGE_ROOM_MAX_H});
        for i in 0..self.room_num{
            let t_room=&self.rooms[i];
            let t_room_l=t_room.x as usize;
            let t_room_r=(t_room.x+t_room.w) as usize;
            let t_room_t=t_room.y as usize;
            let t_room_b=(t_room.y+t_room.h) as usize;
            println!("{},{},{},{}",t_room_l,t_room_t,t_room_r-t_room_l,t_room_b-t_room_t);
            
            for j in t_room_t..t_room_b{
                for k in t_room_l..t_room_r{
                    self.floor[j*(ROUGE_ROOM_MAX_W as usize)  +k]=1;
                }
            }
        }
        println!("");
        for i in 1..self.room_num{
            let t_room1=self.rooms[i-1];
            let t_room2=self.rooms[i];
            self.create_route(
                (t_room1.x+t_room1.w/2.0) as i32,
                (t_room1.y+t_room1.h/2.0) as i32,
                (t_room2.x+t_room2.w/2.0) as i32,
                (t_room2.y+t_room2.h/2.0) as i32
            );
        }
    }

    fn create_room(&mut self,rect:&RectType){
        if ROUGE_ROOM_MAX> self.room_num{
           //部屋を作る条件
           //1.乱数を使用して作成の判定となった
           //2.領域のサイズがこれ以上分割できない
           unsafe{

                if 1==SDL_rand(ROUGE_ROOM_GEN_PARAM) ||
                    ((rect.w as i32)/2)< ROUGE_ROOM_MIN_W ||
                    ((rect.h as i32)/2)<ROUGE_ROOM_MIN_H{
                    let room_w=ROUGE_ROOM_MIN_W+
                        SDL_rand((rect.w as i32)-ROUGE_ROOM_MIN_W);
                    let room_h=ROUGE_ROOM_MIN_H+
                        SDL_rand((rect.h as i32)-ROUGE_ROOM_MIN_H);
                    let l_offset=SDL_rand((rect.w as i32)-room_w);
                    let t_offset=SDL_rand((rect.h as i32)-room_h);
                    self.rooms[self.room_num]=rect_type!{
                        (rect.x as i32)+l_offset,
                        (rect.y as i32)+t_offset,
                        room_w,
                        room_h
                    };
                    self.room_num=self.room_num+1;
                }else{
                    //縦横どちらに分割するか？
                    if 1==SDL_rand(2){
                        //縦に分割する
                        let n_rect1=rect_type!{
                            rect.x+1.0,
                            rect.y+1.0,
                            rect.w-1.0,
                            (rect.h as i32)/2-1
                        };
                        let n_rect2=rect_type!{
                            rect.x+1.0,
                            rect.y+rect.h/2.0+1.0,
                            rect.w-1.0,
                            (rect.h as i32)/2-1
                        };
                        self.create_room(&n_rect1);
                        self.create_room(&n_rect2);
                    }else{
                        //横に分割する
                        let n_rect1=rect_type!{
                            rect.x+1.0,
                            rect.y+1.0,
                            (rect.w/2.0)-1.0,
                            rect.h-1.0
                        };
                        let n_rect2=rect_type!{
                            rect.x+rect.w/2.0+1.0,
                            rect.y+1.0,
                            (rect.w/2.0)-1.0,
                            rect.h-1.0
                        };
                        self.create_room(&n_rect1);
                        self.create_room(&n_rect2);
                    }   
                }
            }
        }

    }
    fn create_route(&mut self,x0:i32,y0:i32,x1:i32,y1:i32){
        let dx=(x1-x0).abs() as f32;
        let dy=(y1-y0).abs() as f32;

        let sx=if x0 < x1 {1}else{-1};
        let sy=if y0 < y1 {1}else{-1};
        let mut err=dx-dy;
        let mut x=x0;
        let mut y=y0;
        let mut e2=0.0;
        while !(x==x1 && y==y1){
            self.floor[(y*ROUGE_ROOM_MAX_W+x) as usize]=1;
            e2=2.0*err;
            if e2 > -dy{
                err=err-dy;
                x=x+sx;
            }
            if e2 < dx{
                err=err+dx;
                y=y+sy;
            }
        }

    }
}
