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
    Pos(usize)

}
pub type NativeFuncType=fn(p_user_data:*mut c_void,state:&mut SB_State)->usize;
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
    Set(usize),
    SetShare(String),
    JmpN(Option<usize>),
    GenShare(String),
    GenVar,
    Return,
    Goto(Option<usize>),
    Gosub(Option<usize>),
    Call(NativeFuncType,usize),
    

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
            Self::Pos(u)=>{
                write!(f,"{}",u)
            }
        }
    }
}