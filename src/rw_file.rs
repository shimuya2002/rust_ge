use std::ffi::*;
use std::ptr::*;
use std::mem::*;
use crate::imports::*;

#[derive(Debug)]
pub struct RW_File{
    sdl_rw_ops:*mut SDL_RWops,
}
impl RW_File{
    pub fn open_read(path:&str)->Result<Self,String>{
        unsafe{
            let path_cstr=CString::new(path).unwrap();
            let mode_cstr=CString::new("r".to_string()).unwrap();
            let ops=SDL_RWFromFile(path_cstr.as_ptr(),mode_cstr.as_ptr());
            if null_mut()!=ops{
                return Ok(
                    Self{
                        sdl_rw_ops:ops
                    }
                );
            }else{
                let err_msg=CStr::from_ptr(SDL_GetError());
                return Err(
                    err_msg.to_str().expect("").to_string()
                );
            }
        }
    }
    pub fn read<T:Default>(&self,read_bytes_size:usize)->Result<Vec<T>,String>{
        unsafe{
            let mut buf=Vec::<T>::new();
            buf.resize_with(read_bytes_size/size_of::<T>(),||{T::default()});
            let read_num=SDL_RWread(self.sdl_rw_ops,
                            buf.as_ptr() as *mut c_void,
                            std::mem::size_of::<T>(),
                            buf.len());
            if 0<read_num{
                return Ok(buf);
            }else{
                let err_msg=CStr::from_ptr(SDL_GetError());
                return Err(
                    err_msg.to_str().expect("").to_string()
                );
            }
        }

    }
    pub fn read_utf8_text(&self)->Result<String,String>{
        let len_res=self.len();
        if let Ok(size)=len_res{
            let read_res=self.read::<u8>(size);
            if let Ok(data)=read_res{
                let conv_str=String::from_utf8(data);
                if let Ok(text)=conv_str{
                    return Ok(text);
                }else{
                    return Err(conv_str.unwrap_err().to_string());
                }
            }else{
                return Err(read_res.unwrap_err());
            }
        }else{
            return Err(len_res.unwrap_err());
        }
    }
    pub fn len(&self)->Result<usize,String>{
        unsafe{
            let file_size=SDL_RWsize(self.sdl_rw_ops);
            if -1!=file_size{
                return Ok(file_size as usize);
            }else{
                let err_msg=CStr::from_ptr(SDL_GetError());
                return Err(
                    err_msg.to_str().expect("").to_string()
                );
            }            
        }
    }
}
impl Drop for RW_File{
    fn drop(&mut self){
        unsafe{
            if null_mut()!=self.sdl_rw_ops{
                SDL_RWclose(self.sdl_rw_ops);
                self.sdl_rw_ops=null_mut();
            }
        }
    }
}
# [cfg(test)]
mod tests{
    use crate::rw_file::*;
    #[test]
    fn test_rwfile(){
        let file=RW_File::open_read("./assets/test.txt");
        if let Ok(f)=file{
            if let Ok(s)=f.len(){
                assert!(0<s);
                if let Ok(t)=f.read::<u8>(s){
                    assert_eq!(s,t.len());
                    assert_eq!('a',t[0] as char);
                }else{
                    assert!(false);
                }
            }else{
                assert!(false);
            }

        }else{
            assert!(false);
        }
    }

}
