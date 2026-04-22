use std::ffi::*;
use crate::imports::*;
use crate::sb_state::*;
use crate::sb_cmdtype::*;
use crate::game_app::*;
use crate::config::*;
use crate::geometory::*;
use crate::sprite::*;
use crate::sprite::*;
include!("./geometory_inc.rs");

pub fn load(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let path=state.value_to_string(0);
        let pos=state.value_to_int(1);
        if LEFT_IMAGE==pos{
            (*app).left_image=path.clone();
            (*app).app.set_gpage(BUSTUP_GPAGE,RENDER_GPAGE);
            (*app).app.load_image(0,0,path.as_str());
            (*app).show_left=None;
        }else if RIGHT_IMAGE==pos{
            (*app).right_image=path.clone();
            (*app).app.set_gpage(BUSTUP_GPAGE,RENDER_GPAGE);
            (*app).app.load_image(WND_W/2,0,path.as_str());
            (*app).show_right=None;

        }else if BG_IMAGE==pos{
            (*app).bg_image=path.clone();
            (*app).app.set_gpage(BG_GPAGE,RENDER_GPAGE);
            (*app).app.load_image(0,0,path.as_str());
            (*app).show_bg=None;

        }else if PLAYER_CHARA_IMAGE==pos{
            (*app).player_battle_image=path.clone();
            (*app).app.set_gpage(PLAYER_CHARA_GPAGE,RENDER_GPAGE);
            (*app).app.load_image(0,0,path.as_str());
            (*app).show_player_battle=None;

        }
        (*app).app.set_gpage(0,0);
        return None;
    }
}
pub fn show(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let pos=state.value_to_int(0);
        if LEFT_IMAGE==pos{
            (*app).show_left=Some(RENDER_MODE::Norm);
        }else if RIGHT_IMAGE==pos{
            (*app).show_right=Some(RENDER_MODE::Norm);

        }else if BG_IMAGE==pos{
            (*app).show_bg=Some(RENDER_MODE::Norm);

        }else if PLAYER_CHARA_IMAGE==pos{
            (*app).show_player_battle=Some(RENDER_MODE::Norm);

        }
        state.run_script=false;
        return None;
        
    }
}
pub fn movein(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let pos=state.value_to_int(0);
        if LEFT_IMAGE==pos{
            (*app).show_left=Some(RENDER_MODE::MoveinFromLeft(0));
        }else if RIGHT_IMAGE==pos{
            (*app).show_right=Some(RENDER_MODE::MoveinFromRight(0));

        }else if BG_IMAGE==pos{
            (*app).show_bg=Some(RENDER_MODE::MoveinFromLeft(0));

        }
        state.run_script=false;
        return None;
        
    }
}
pub fn moveout(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let pos=state.value_to_int(0);
        if LEFT_IMAGE==pos{
            (*app).show_left=Some(RENDER_MODE::MoveoutToLeft(0));
        }else if RIGHT_IMAGE==pos{
            (*app).show_right=Some(RENDER_MODE::MoveoutToRight(0));

        }else if BG_IMAGE==pos{
            (*app).show_bg=Some(RENDER_MODE::MoveoutToLeft(0));

        }
        state.run_script=false;
        return None;
        
    }
}
pub fn text(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let txt=state.value_to_string(0);
        (*app).text_log[(*app).text_log_pos]=txt.clone();
        (*app).text_log_pos=((*app).text_log_pos+1)%(*app).text_log.len();
        (*app).text_changed=true;
        state.run_script=false;
        return None;
    }
}
pub fn wait(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let ms=state.value_to_int(0);
        (*app).app.wait(ms as u32);
        return None;
    }
}
pub fn create_sprite(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let y_offset=state.value_to_int(0);
        let x_offset=state.value_to_int(1);
        let y_origin=state.value_to_int(2);
        let x_origin=state.value_to_int(3);
        let g_page=state.value_to_int(4);
        println!("{},{},{},{},{}",g_page,x_origin,y_origin,x_offset,y_offset);
        let sprite=Sprite{
            rect:rect_type!{x_origin,y_origin,x_offset,y_offset},
            gpage:g_page as usize
        };
        (*app).app.sprites.push(sprite);
        return Some(VarType::Pos((*app).app.sprites.len()));

    }
}
pub fn selection(p_user_data:*mut c_void,state:&mut SB_State)->Option<VarType>{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let msg4=state.value_to_string(0);
        let msg3=state.value_to_string(1);
        let msg2=state.value_to_string(2);
        let msg1=state.value_to_string(3);
        let mut size=(*app).app.measure_msg_utf8(msg4.as_str());
# [cfg(feature="use_sdl3")]
        let sw=size.w as f32;
# [cfg(feature="use_sdl2")]
        let sw=size.w;
# [cfg(feature="use_sdl3")]
        let sh=size.h as f32;
# [cfg(feature="use_sdl2")]
        let sh=size.h;


        let rect4=rect_type!{MSG_TEXT_SIZE,WND_H-size.h,size.w,size.h};
        size=(*app).app.measure_msg_utf8(msg3.as_str());
        let rect3=rect_type!{MSG_TEXT_SIZE,rect4.y-sh,size.w,size.h};
        size=(*app).app.measure_msg_utf8(msg2.as_str());
        let rect2=rect_type!{MSG_TEXT_SIZE,rect3.y-sh,size.w,size.h};
        size=(*app).app.measure_msg_utf8(msg1.as_str());
        let rect1=rect_type!{MSG_TEXT_SIZE,rect2.y-sh,size.w,size.h};
        (*app).selections=Some(
            [
                SelectionInfo{
                    msg:msg1,
                    rect:rect1,
                },
                SelectionInfo{
                    msg:msg2,
                    rect:rect2,
                },
                SelectionInfo{
                    msg:msg3,
                    rect:rect3,
                },
                SelectionInfo{
                    msg:msg4,
                    rect:rect4,
                },
            ]
        );
        (*app).text_changed=true;
        return None;
    }
}