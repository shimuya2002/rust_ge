use std::ffi::*;
use std::ptr::*;
use std::rc::*;
use std::borrow::*;
use std::cmp::*;
use crate::imports::*;
use crate::config::*;
use crate::cache_tbl::*;
use crate::texture::*;
use crate::font::*;
use crate::size::*;
use crate::anim_set::*;
pub type AppEventAction=fn(p_user_data:*mut c_void);

pub struct App{
# [cfg(not(feature="non_bindings"))]
    pub p_app:*mut c_void,
    g_pages:Vec<*mut SDL_Texture>,
    anim_sets:Vec<AnimSet>,
    render_page:usize,
    display_page:usize,
    image_cache:CacheTbl<Texture>,
    sdl_renderer:*mut SDL_Renderer,
    sdl_window:*mut SDL_Window,
    sdl_event:SDL_Event,
    sdl_fps_manager:FPSmanager,
    button_buf_idx:usize,
    button_state_buf:[[bool;BUTTON_NUM];2],
    mouse_button_state_buf:[bool;2],
    click_pos:SDL_Point,
    msg_font:Option<Font>,
    ui_font:Option<Font>,
    p_ud:*mut c_void,
    on_init:Option<AppEventAction>,
    on_term:Option<AppEventAction>,
    pub dirty_rect_tbl:Vec<SDL_Rect>
}

impl App{
    pub fn new()->Self{

        unsafe{
            let p_app=if cfg!(feature="non_bindings"){
                SDL_Init(SDL_INIT_EVERYTHING);
                IMG_Init((IMG_InitFlags_IMG_INIT_PNG | IMG_InitFlags_IMG_INIT_JPG) as i32 /*| IMG_INIT_WEBP*/);
                TTF_Init();
                null_mut()
            }else{
                app_init()
            };
            return Self{
# [cfg(not(feature="non_bindings"))]
                p_app:p_app,
                g_pages:Vec::new(),
                anim_sets:Vec::new(),
                render_page:0,
                display_page:0,
                image_cache:CacheTbl::new(),
                sdl_renderer:null_mut(),
                sdl_window:null_mut(),
                sdl_event:SDL_Event{type_:0},
                sdl_fps_manager:FPSmanager{framecount:0,rateticks:0.0,baseticks:0,lastticks:0,rate:0},
                button_buf_idx:0,
                button_state_buf:[
                    [false;BUTTON_NUM],
                    [false;BUTTON_NUM]
                ],
                mouse_button_state_buf:[false,false],
                click_pos:SDL_Point{x:0,y:0},
                msg_font:None,
                ui_font:None,
                p_ud:null_mut(),
                on_init:None,
                on_term:None,
                dirty_rect_tbl:Vec::new()
            };
            

        }
    }
    pub fn set_init_event(&mut self,f:AppEventAction){
        self.on_init=Some(f);
    }
    pub fn set_term_event(&mut self,f:AppEventAction){
        self.on_term=Some(f);
    }
    pub fn set_ud(&mut self,p_ud:*mut c_void){
        self.p_ud=p_ud;
    }
    pub fn run_step(&mut self,w:i32,h:i32)->bool{
        unsafe{
            if cfg!(feature="non_bindings"){
                if null_mut()==self.sdl_window{
                    let caption_cstr=CString::new("").unwrap();
                    self.sdl_window=SDL_CreateWindow(
                        caption_cstr.as_ptr(),
                        SDL_WINDOWPOS_CENTERED_MASK as i32,
                        SDL_WINDOWPOS_CENTERED_MASK as i32,
                        w,h,
                        SDL_WindowFlags_SDL_WINDOW_OPENGL
                    );
                    if null_mut()==self.sdl_window{
                        return false;
                    }
                    self.sdl_renderer=SDL_CreateRenderer(
                        self.sdl_window,
                        -1,
                        SDL_RendererFlags_SDL_RENDERER_ACCELERATED |
                        SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE
                    );
                    if null_mut()==self.sdl_renderer{
                        SDL_DestroyWindow(self.sdl_window);
                        self.sdl_window=null_mut();
                        return false;
                    }
                    self.init_resources(w,h);
                    SDL_initFramerate(&mut self.sdl_fps_manager);
                    SDL_setFramerate(&mut self.sdl_fps_manager,DEF_FRAME_RATE);
                    if let Some(f)=self.on_init{
                        f(self.p_ud);
                    }
                    return true;
                }else{
                    SDL_framerateDelay(&mut self.sdl_fps_manager);
                    self.update_screen(w,h);
                    self.mouse_button_state_buf[self.button_buf_idx]=false;


                    while 1==SDL_PollEvent(&mut self.sdl_event){
                        match self.sdl_event.type_{
                            SDL_EventType_SDL_KEYDOWN=>{

                                match self.sdl_event.key.keysym.sym{
                                    SDL_KeysymType_SDLK_UP=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_UP]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_DOWN=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_DOWN]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_LEFT=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_LEFT]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_RIGHT=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_RIGHT]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_z=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_A]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_x=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_B]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_a=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_X]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_s=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_Y]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_q=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_L]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_w=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_R]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_1=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_ZL]
                                            =true;
                                    },
                                    SDL_KeysymType_SDLK_2=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_ZR]
                                            =true;
                                    },
                                 }

                            },
                            SDL_EventType_SDL_KEYUP=>{

                                match self.sdl_event.key.keysym.sym{
                                    SDL_KeysymType_SDLK_UP=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_UP]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_DOWN=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_DOWN]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_LEFT=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_LEFT]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_RIGHT=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_RIGHT]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_z=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_A]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_x=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_B]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_a=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_X]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_s=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_Y]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_q=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_L]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_w=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_R]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_1=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_ZL]
                                            =false;
                                    },
                                    SDL_KeysymType_SDLK_2=>{
                                        self.button_state_buf[self.button_buf_idx][BUTTON_ZR]
                                            =false;
                                    },
                                 }

                            },
                            SDL_EventType_SDL_MOUSEBUTTONDOWN=>{
                                if 0!=((SDL_BUTTON_LEFT as u8) & self.sdl_event.button.button){
                                    self.mouse_button_state_buf[self.button_buf_idx]=true;
                                    self.click_pos.x=self.sdl_event.button.x;
                                    self.click_pos.y=self.sdl_event.button.y;
                                }

                            },
                            SDL_EventType_SDL_QUIT=>{
                                if let Some(f)=self.on_term{
                                    f(self.p_ud);
                                }
                                self.deinit_resources();
                                SDL_DestroyRenderer(self.sdl_renderer);
                                SDL_DestroyWindow(self.sdl_window);
                                self.sdl_renderer=null_mut();
                                self.sdl_window=null_mut();
                                return false;
                            },
                            _=>{}
                        }
                    

                    }
                    self.button_buf_idx=(self.button_buf_idx+1)&1;
                    return true;
                }
            }else{
 # [cfg(not(feature="non_bindings"))]
                return 0!=run_step(self.p_app,w,h);
 # [cfg(feature="non_bindings")]
                return false;
            };

        }
    }
    ///* ライブラリ固有のリソースを初期化する
    ///* 'w' 画面の横幅
    ///* 'h' 画面の縦幅
    pub fn init_resources(&mut self,w:i32,h:i32){
        for i in 0..G_PAGE_NUM{
            unsafe{
 # [cfg(feature="non_bindings")]
                let renderer=self.sdl_renderer;
 # [cfg(not(feature="non_bindings"))]
                let renderer=get_sdl_renderer(self.p_app),
 # [cfg(feature="use_sdl3")]
                let tex_access:u32=SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET;
 # [cfg(feature="use_sdl2")]
                let tex_access:i32=SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET;
                let tex=SDL_CreateTexture(renderer,
                            SDL_PixelFormatEnum_SDL_PIXELFORMAT_ARGB8888 as u32,//SDL_PIXELFORMAT_UNKNOWN,
                            tex_access,
                            w,
                            h);
                self.g_pages.push(tex);
                self.dirty_rect_tbl.push(SDL_Rect{x:0,y:0,w:0,h:0});
            }

        }
        let msg_font_r=Font::new(FONT_FILE_PATH,MSG_TEXT_SIZE);
        if let Ok(font)=msg_font_r{
            self.msg_font=Some(font);
        }else if let Err(msg)=msg_font_r{
            println!("{}",msg);
        }
        let ui_font_r=Font::new(FONT_FILE_PATH,UI_TEXT_SIZE);
        if let Ok(font)=ui_font_r{
            self.ui_font=Some(font);
        }else if let Err(msg)=ui_font_r{
            println!("{}",msg);
        }
        for i in 0..ANIM_SET_NUM{
            self.anim_sets.push(AnimSet::default());
        }
    }
    pub fn deinit_resources(&mut self){
        self.ui_font=None;
        self.msg_font=None;
        self.image_cache.clear();
        for i in &self.g_pages{
            unsafe{
                if null_mut()!=*i{
                    SDL_DestroyTexture(*i);

                }
            }
        }
        self.g_pages.clear();
        self.dirty_rect_tbl.clear();
    }

    pub fn quit(&self){
        unsafe{
            SDL_PushEvent(
                &mut SDL_Event{
                    quit:SDL_QuitEvent{
                        type_:SDL_EventType_SDL_QUIT,
                        timestamp:SDL_GetTicks(),
                        reserved:0
                    }

                });
        }
    }
}
impl Drop for App{
    fn drop(&mut self){
        unsafe{
 # [cfg(feature="non_bindings")]
                TTF_Quit();
 # [cfg(feature="non_bindings")]
                IMG_Quit();
 # [cfg(feature="non_bindings")]
                SDL_Quit();

 # [cfg(not(feature="non_bindings"))]
                if null_mut()!=self.p_app
                {
 # [cfg(not(feature="non_bindings"))]
                        app_quit(self.p_app);
 # [cfg(not(feature="non_bindings"))]
                        self.p_app=null_mut();

                    
                }

            
        }
    }

    
}
include!("./app_draw.rs");
