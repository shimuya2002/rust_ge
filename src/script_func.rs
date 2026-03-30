use std::ffi::*;
use crate::imports::*;
use crate::sb_state::*;
use crate::game_app::*;
use crate::config::*;
use crate::size::*;
use crate::anim_set::*;
pub fn load(p_user_data:*mut c_void,state:&mut SB_State)->usize{
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
        return 0;
    }
}
pub fn show(p_user_data:*mut c_void,state:&mut SB_State)->usize{
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
        return 0;
        
    }
}
pub fn movein(p_user_data:*mut c_void,state:&mut SB_State)->usize{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let pos=state.value_to_int(0);
        if LEFT_IMAGE==pos{
            (*app).show_left=Some(RENDER_MODE::MoveinFromLeft(0));
        }else if RIGHT_IMAGE==pos{
            (*app).show_right=Some(RENDER_MODE::MoveinFromRight(0));

        }else if BG_IMAGE==pos{
            (*app).show_bg=Some(RENDER_MODE::MoveinFromLeft(0));
            return 0;
        }
        state.run_script=false;
        return 0;
        
    }
}
pub fn moveout(p_user_data:*mut c_void,state:&mut SB_State)->usize{
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
        return 0;
        
    }
}
pub fn text(p_user_data:*mut c_void,state:&mut SB_State)->usize{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let txt=state.value_to_string(0);
        (*app).text_log[(*app).text_log_pos]=txt.clone();
        (*app).text_log_pos=((*app).text_log_pos+1)%(*app).text_log.len();
        (*app).text_changed=true;
        state.run_script=false;
        return 0;
    }
}
pub fn wait(p_user_data:*mut c_void,state:&mut SB_State)->usize{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let ms=state.value_to_int(0);
        (*app).app.wait(ms as u32);
        return 0;
    }
}
pub fn create_anim_set(p_user_data:*mut c_void,state:&mut SB_State)->usize{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let y_offset=state.value_to_int(0);
        let x_offset=state.value_to_int(1);
        let y_origin=state.value_to_int(2);
        let x_origin=state.value_to_int(3);
        let frame_num=state.value_to_int(4);
        let g_page=state.value_to_int(5);
        let name=state.value_to_string(6);
        let anim_idx=state.value_to_int(7);
        let anim_set=AnimSet::new(
            name,
            g_page as usize,
            &SDL_Rect{x:x_offset,y:y_offset,w:x_offset,h:y_offset},
            &Size{w:x_offset,h:y_offset},
            frame_num
        );
        (*app).app.set_anim_set(anim_idx as usize,anim_set);
        return 0;

    }
}
pub fn selection(p_user_data:*mut c_void,state:&mut SB_State)->usize{
    unsafe{
        let mut app=p_user_data as *mut GameApp;
        let msg4=state.value_to_string(0);
        let msg3=state.value_to_string(1);
        let msg2=state.value_to_string(2);
        let msg1=state.value_to_string(3);
        let mut size=(*app).app.measure_msg_utf8(msg4.as_str());
        let rect4=SDL_Rect{x:MSG_TEXT_SIZE,y:WND_H-size.h,w:size.w,h:size.h};
        size=(*app).app.measure_msg_utf8(msg3.as_str());
        let rect3=SDL_Rect{x:MSG_TEXT_SIZE,y:rect4.y-size.h,w:size.w,h:size.h};
        size=(*app).app.measure_msg_utf8(msg2.as_str());
        let rect2=SDL_Rect{x:MSG_TEXT_SIZE,y:rect3.y-size.h,w:size.w,h:size.h};
        size=(*app).app.measure_msg_utf8(msg1.as_str());
        let rect1=SDL_Rect{x:MSG_TEXT_SIZE,y:rect2.y-size.h,w:size.w,h:size.h};
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
        return 0;
    }
}