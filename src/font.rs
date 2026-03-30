use std::ffi::*;
use std::ptr::*;
use crate::imports::*;
use crate::size::*;
pub struct Font{
    sdl_font:*mut TTF_Font
}

impl Font{
    ///* コンストクタ
    ///* 'path' フォントへのファイルパス
    ///* 'size' フォントサイズ
    pub fn new(path:&str,size:i32)->Result<Self,String>{
        unsafe{
            let path_cstr=CString::new(path).expect("");
 # [cfg(feature="use_sdl3")]
            let font=TTF_OpenFont(path_cstr.as_ptr(),size as f32);
 # [cfg(feature="use_sdl2")]
            let font=TTF_OpenFont(path_cstr.as_ptr(),size);
            if null_mut()==font{
                let err_msg=CStr::from_ptr(SDL_GetError());
                return Err(
                    err_msg.to_str().expect("").to_string()
                );
            }
            return Ok(Self{
                sdl_font:font
            });

        }
    }

    ///* テキスト描画に必要な領域を求める
    ///* 'txt' 描画する文字列
    pub fn measure_utf8_size(&self,txt:&str)->Size{
        unsafe{
            let mut r_size=Size{w:0,h:0};
            let txt_cstr=CString::new(txt).expect("");
 # [cfg(feature="use_sdl3")]
            TTF_GetStringSize(self.sdl_font,
                txt_cstr.as_ptr(),
                txt_cstr.count_bytes(),
                &mut r_size.w,
                &mut r_size.h);

 # [cfg(feature="use_sdl2")]
            TTF_SizeUTF8(self.sdl_font,txt_cstr.as_ptr(),&mut r_size.w,&mut r_size.h);
                    
            return r_size;
        }
    }
    ///* UTF8テキストを描画する
    ///* 'txt' 描画する文字列
    ///* 'wrap_w' 文字列の折り返しを行う横幅
    ///* 'result' 描画済みのSDL_Surface
    pub fn render_utf8(&self,txt:&str,wrap_w:u32)->*mut SDL_Surface{
        unsafe{
            let txt_cstr=CString::new(txt).expect("");
            let clr=SDL_Color{r:255,g:255,b:255,a:255};
 # [cfg(feature="use_sdl3")]
            return TTF_RenderText_Blended_Wrapped(self.sdl_font,
                txt_cstr.as_ptr(),
                txt_cstr.count_bytes(),
                clr,
                wrap_w as i32);
 # [cfg(feature="use_sdl2")]
            return TTF_RenderUTF8_Blended_Wrapped(self.sdl_font,txt_cstr.as_ptr(),clr,wrap_w);
        }
    }
}
impl Drop for Font{
    fn drop(&mut self){
        if null_mut()!=self.sdl_font{
            unsafe{
                TTF_CloseFont(self.sdl_font);
                self.sdl_font=null_mut();

            }
        }
    }
}