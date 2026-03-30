
use std::ffi::*;
use std::ptr::*;


use crate::imports::*;
pub struct Texture{
    pub tex:*mut SDL_Texture,
    pub w:i32,
    pub h:i32
}
impl Texture{
    pub fn load(renderer:*mut SDL_Renderer,path:&str)->Result<Self,String>{
        unsafe{

            let file_path=CString::new(path).expect("");
            let p_surf=IMG_Load(file_path.as_ptr());
            if null_mut()==p_surf{
                return Err(format!("Failed to load {}",path).to_string());
            }
            let pTex=SDL_CreateTextureFromSurface(renderer,p_surf);
            if null_mut()==pTex{
                return Err(format!("Failed to create texture {}",path).to_string());

            }
            let sw=(*p_surf).w;
            let sh=(*p_surf).h;
            SDL_FreeSurface(p_surf);
            return Ok(Self{
                tex:pTex,
                w:sw,
                h:sh
            });

        }
    }
    pub fn from_surface(renderer:*mut SDL_Renderer,p_surf:*mut SDL_Surface)->Result<Self,String>{
        unsafe{
            let pTex=SDL_CreateTextureFromSurface(renderer,p_surf);
            if null_mut()==pTex{
                let err_msg=CStr::from_ptr(SDL_GetError());
                return Err(
                    err_msg.to_str().expect("").to_string()
                );

            }
            let sw=(*p_surf).w;
            let sh=(*p_surf).h;
            return Ok(Self{
                tex:pTex,
                w:sw,
                h:sh
            });
        }
    }
}
impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            if null_mut()!=self.tex{
                SDL_DestroyTexture(self.tex);
                self.tex=null_mut();
            }

        }
    }
}