use std::fmt;
use std::ffi::*;
use crate::sb_state::*;
///BASICコマンドに関する定義を記載したモジュール

#[derive(Clone)]
pub enum VarType{
    None,
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    VarPos(usize),
    SharePos(String),
    JmpPos(usize),
    //Rect(SDL_Rect)


}
pub type NativeFuncType=fn(p_user_data:*mut c_void,state:&mut SB_State,arg_num:i32)->Option<VarType>;
#[derive(Debug,PartialEq)]
pub enum CmdType{
    Nop,
    PushStr(String),
    PushInt(i32),
    PushFloat(f32),
    PushBool(bool),
    PushVar(usize),
    PushShare(String),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Set,
    SetShare(String),
    JmpN(Option<usize>),
    GenShare(String),
    GenVar,
    Return,
    Goto(Option<usize>),
    Gosub(Option<usize>),
    Call(NativeFuncType,usize),
    Pop,
    

}

impl fmt::Display for VarType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::None=>{
                 write!(f,"None")
            },
            Self::Int(i)=>{
                 write!(f,"{}",i)

            }
            Self::Float(fl)=>{
                 write!(f,"{}",fl)

            },
            Self::Str(s)=>{
                 write!(f,"\"{}\"",s)

            },
            Self::Bool(b)=>{
                 write!(f,"{}",b)

            },
            Self::VarPos(u)=>{
                write!(f,"{}",u)
            }
            Self::SharePos(u)=>{
                write!(f,"{}",u)
            }
            Self::JmpPos(u)=>{
                write!(f,"{}",u)
            },

        }
    }
}

impl fmt::Display for CmdType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Nop=>{
                write!(f,"nop")
            },
            Self::PushStr(s)=>{
                write!(f,"push str {}",s)
            },
            Self::PushInt(i)=>{
                write!(f,"push int {}",i)
            },
            Self::PushFloat(fl)=>{
                write!(f,"push float {}",fl)
            },
            Self::PushBool(b)=>{
                write!(f,"push bool {}",b)
            },
            Self::PushVar(idx)=>{
                write!(f,"push var {}",idx)
            },
            Self::PushShare(key)=>{
                write!(f,"push share {}",key)
            },
            Self::Add=>{
                write!(f,"add")
            },
            Self::Sub=>{
                write!(f,"sub")
            },
            Self::Mul=>{
                write!(f,"mul")
            },
            Self::Div=>{
                write!(f,"div")
            },
            Self::Mod=>{
                write!(f,"mod")
            },
            Self::And=>{
                write!(f,"and")

            },
            Self::Or=>{
                write!(f,"or")
            },
            Self::Set=>{
                write!(f,"set")
            },
            Self::SetShare(key)=>{
                write!(f,"set share {}",key)
            },
            Self::JmpN(idx)=>{
                write!(f,"jmp n {}",
                    if let Some(dst_idx)=idx{dst_idx.to_string()}else{"None".to_string()})

            },
            Self::GenShare(key)=>{
                write!(f,"gen share \"{}\"",key)
            },
            Self::GenVar=>{
                write!(f,"gen var")
            },
            Self::Return=>{
                write!(f,"return")
            },
            Self::Goto(idx)=>{
                write!(f,"goto {}",
                    if let Some(dst_idx)=idx{dst_idx.to_string()}else{"None".to_string()})

            },
            Self::Gosub(idx)=>{
                write!(f,"go sub {}",
                    if let Some(dst_idx)=idx{dst_idx.to_string()}else{"None".to_string()})
            },
            Self::Call(func,arg_num)=>{
                write!(f,"call {:x},{}",func as *const _ as *const c_void as usize,arg_num)
            },
            Self::Pop=>{
                write!(f,"pop")
            },
        }
    }
}