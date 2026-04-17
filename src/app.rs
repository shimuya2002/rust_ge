#[macro_use]
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
use crate::geometory::*;
use crate::anim_set::*;
include!("./geometory_inc.rs");

/// アプリケーション内で発生したイベントを処理するハンドラー
///* 'p_user_data' Appオブジェクトに設定したユーザデータ
pub type AppEventAction=fn(p_user_data:*mut c_void);
/// アプリケーションオブジェクト
pub struct App{
# [cfg(not(feature="non_bindings"))]
    pub p_app:*mut c_void,
    ///グラフィックページ
    g_pages:Vec<*mut SDL_Texture>,
    ///管理するアニメーションセット
    anim_sets:Vec<AnimSet>,
    ///描画先のグラフィックページ
    render_page:usize,
    ///画面に表示されるグラフィックページ
    display_page:usize,
    ///画像キャッシュ
    image_cache:CacheTbl<Texture>,
    ///SDL2/SDL3用のレンダラー
    sdl_renderer:*mut SDL_Renderer,
    ///SDL2/SDL3用ウィンドウ
    sdl_window:*mut SDL_Window,
    ///SDL2/SDL3用イベント
    sdl_event:SDL_Event,
    ///SDL2/SDL3用FPS同期オブジェクト
    sdl_fps_manager:FPSmanager,
    ///ボタン押下状態バッファー参照用のインデックス
    button_buf_idx:usize,
    ///ボタン押下状態保存用のバッファー
    button_state_buf:[[bool;BUTTON_NUM];2],
    ///マウスボタン押下状態保存用のバッファー
    mouse_button_state_buf:[bool;2],
    ///マウスポインタの位置
    click_pos:SDL_Point,
    ///画面テキスト用のフォント
    msg_font:Option<Font>,
    ///UI用のフォント
    ui_font:Option<Font>,
    ///イベントハンドラに渡すユーザデータ
    p_ud:*mut c_void,
    ///アプリケーション初期化時に呼ばれるハンドラー
    on_init:Option<AppEventAction>,
    ///アプリケーション終了時に呼ばれるハンドラー
    on_term:Option<AppEventAction>,
    ///各グラフィックの更新領域
    pub dirty_rect_tbl:Vec<RectType>
}

impl App{
    ///コンストラクタ
    pub fn new()->Self{

        unsafe{
            let p_app=if cfg!(feature="non_bindings"){
                SDL_Init(SDL_INIT_EVERYTHING);
 # [cfg(feature="use_sdl2")]
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
    ///初期化イベントハンドラの設定
    ///* 'f' 設定するイベントハンドラ
    pub fn set_init_event(&mut self,f:AppEventAction){
        self.on_init=Some(f);
    }
    ///終了イベントハンドラを設定する
    ///* 'f' 設定するイベントハンドラ
    pub fn set_term_event(&mut self,f:AppEventAction){
        self.on_term=Some(f);
    }
    ///ユーザデータを設定する
    ///* 'p_ud' 設定するユーザデータ
    pub fn set_ud(&mut self,p_ud:*mut c_void){
        self.p_ud=p_ud;
    }
    ///イベント読み出しを行う
    /// 処理すべきイベントが残っているならtrue、それ以外ならfalseを返す
    fn poll_event(&mut self)->bool{
        unsafe{
 # [cfg(feature="use_sdl3")]
            return SDL_PollEvent(&mut self.sdl_event);
 # [cfg(feature="use_sdl2")]
            return 1==SDL_PollEvent(&mut self.sdl_event);

        }
    }
    ///現在のイベントキュー内のイベント処理を行う
    ///* 'w' 表示するウィンドウの横幅
    ///* 'h' 表示するウィンドウの縦幅
    pub fn run_step(&mut self,w:i32,h:i32)->bool{
        unsafe{
            if cfg!(feature="non_bindings"){
                //ウィンドウが生成されていならウィンドウとレンダラーを生成する
                if null_mut()==self.sdl_window{
                    println!("Begin window create");
                    let caption_cstr=CString::new("").unwrap();
 #[cfg(feature="use_sdl3")]
                    let window=SDL_CreateWindow(
                        caption_cstr.as_ptr(),
                        w,h,
                        SDL_WindowFlags_SDL_WINDOW_OPENGL
                    );
 #[cfg(feature="use_sdl2")]
                    let window=SDL_CreateWindow(
                        caption_cstr.as_ptr(),
                        SDL_WINDOWPOS_CENTERED_MASK as i32,
                        SDL_WINDOWPOS_CENTERED_MASK as i32,
                        w,h,
                        SDL_WindowFlags_SDL_WINDOW_OPENGL
                    );
                    
                    self.sdl_window=window;
                    if null_mut()==self.sdl_window{
                        println!("Failed create window!");
                        return false;
                    }
 #[cfg(feature="use_sdl3")]
                    let mut renderer=SDL_CreateGPURenderer(null_mut(),self.sdl_window);
                    if null_mut()==renderer{
                        renderer=SDL_CreateRenderer(self.sdl_window,null_mut());
                    }

                    

 #[cfg(feature="use_sdl2")]
                    let renderer=SDL_CreateRenderer(self.sdl_window,-1,SDL_RendererFlags_SDL_RENDERER_ACCELERATED |SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE);
                    
                    self.sdl_renderer=renderer;
                    if null_mut()==self.sdl_renderer{
                        let err_msg=CStr::from_ptr(SDL_GetError());
                        println!("Failed to create renderer {}",err_msg.to_str().expect("").to_string());
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
                    self.update_screen();
                    self.mouse_button_state_buf[self.button_buf_idx]=false;


                    while self.poll_event(){
 # [cfg(target_os="windows")]                        
                        let event_type=self.sdl_event.type_ as i32;
 # [cfg(not(target_os="windows"))]                        
                        let event_type=self.sdl_event.type_;

                        match event_type{
                            SDL_QUIT_EVENT_VALUE=>{
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
                            SDL_KEYDOWN_EVENT_VALUE=>{

 # [cfg(feature="use_sdl3")]
                                let key_code=self.sdl_event.key.key;
 # [cfg(feature="use_sdl2")]
                                let key_code=self.sdl_event.key.keysym.sym;
                                self.proc_keydown(key_code);

                            },
                            SDL_KEYUP_EVENT_VALUE=>{
 # [cfg(feature="use_sdl3")]
                                let key_code=self.sdl_event.key.key;
 # [cfg(feature="use_sdl2")]
                                let key_code=self.sdl_event.key.keysym.sym;
                                self.proc_keyup(key_code);
                            },
                            SDL_MOUSE_BUTTON_DOWN_EVENT_VALUE=>{
                                //保存しているマウスの状態を更新する
                                if 0!=((SDL_BUTTON_LEFT as u8) & self.sdl_event.button.button){
                                    self.mouse_button_state_buf[self.button_buf_idx]=true;
                                    self.click_pos.x=self.sdl_event.button.x as i32;
                                    self.click_pos.y=self.sdl_event.button.y as i32;
                                }

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
                let renderer=get_sdl_renderer(self.p_app);

# [cfg(feature="use_sdl3")]
                let pix_format:SDL_PixelFormat=SDL_PixelFormat_SDL_PIXELFORMAT_ARGB8888;


                
# [cfg(feature="use_sdl2")]
                let pix_format:i32=SDL_PixelFormatEnum_SDL_PIXELFORMAT_ARGB8888 as u32;



# [cfg(feature="use_sdl3")]
                let tex_access:SDL_TextureAccess=SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET;
 # [cfg(feature="use_sdl2")]
                let tex_access:i32=SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET as i32;


                let tex=SDL_CreateTexture(renderer,
                            pix_format,//SDL_PIXELFORMAT_UNKNOWN,
                            tex_access,
                            w,
                            h);
                self.g_pages.push(tex);
                self.dirty_rect_tbl.push(rect_type!{0,0,0,0});
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
    ///ライブラリ固有のリソースを破棄する
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
    /// キー押下解除処理を行う
    ///* 'key_code' 押下されたキーコード
    fn proc_keyup(&mut self,key_code:u32){
        match key_code{
            SDL_K_UP_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_UP]=false;
            },
            SDL_K_DOWN_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_DOWN]=false;
            },
            SDL_K_LEFT_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_LEFT]=false;
            },
            SDL_K_RIGHT_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_RIGHT]=false;
            },
            SDL_K_z_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_A]=false;
            },
            SDL_K_x_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_B]=false;
            },
            SDL_K_a_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_X]=false;
            },
            SDL_K_s_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_Y]=false;
            },
            SDL_K_q_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_L]=false;
            },
            SDL_K_w_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_R]=false;
            },
            SDL_K_1_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_ZL]=false;
            },
            SDL_K_2_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_ZR]=false;
            },
            _=>{},
        }


    }
    /// キー押下処理を行う
    ///* 'key_code' 押下されたキーコード
    fn proc_keydown(&mut self,key_code:u32){
        match key_code{
            SDL_K_UP_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_UP]=true;
            },
            SDL_K_DOWN_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_DOWN]=true;
            },
            SDL_K_LEFT_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_LEFT]=true;
            },
            SDL_K_RIGHT_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_RIGHT]=true;
            },
            SDL_K_z_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_A]=true;
            },
            SDL_K_x_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_B]=true;
            },
            SDL_K_a_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_X]=true;
            },
            SDL_K_s_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_Y]=true;
            },
            SDL_K_q_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_L]=true;
            },
            SDL_K_w_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_R]=true;
            },
            SDL_K_1_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_ZL]=true;
            },
            SDL_K_2_VALUE=>{
                self.button_state_buf[self.button_buf_idx][BUTTON_ZR]=true;
            },
            _=>{},
        }

    }
    ///アプリケーションを終了させる
    pub fn quit(&self){
        unsafe{
            SDL_PushEvent(
                &mut SDL_Event{
                    quit:SDL_QuitEvent{
 # [cfg(feature="use_sdl3")]
                        type_:SDL_EventType_SDL_EVENT_QUIT,
 # [cfg(feature="use_sdl2")]
                        type_:SDL_EventType_SDL_QUIT,

                        timestamp:SDL_GetTicks(),
 # [cfg(feature="use_sdl3")]
                        reserved:0
                    }

                });
        }
    }
}
impl Drop for App{
    ///ドロップ処理
    fn drop(&mut self){
        unsafe{
 # [cfg(feature="non_bindings")]
                TTF_Quit();
 # [cfg(feature="non_bindings")]
 # [cfg(feature="use_sdl2")]
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
include!("./app_draw_sdl3.rs");
include!("./app_draw_sdl2.rs");
include!("./app_input.rs");
