use std::ffi::*;
use std::ptr::*;
use std::collections::*;
use crate::config::*;
use crate::sb_cmdtype::*;
use crate::sb_syntax::*;
use crate::rw_file::*;
use crate::script_func::*;
pub struct SB_State{
    cmd_buf:Vec<CmdType>,
    cmd_pos:usize,
    pub stack:Vec<VarType>,
    pub p_user_data:*mut c_void,
    pub run_script:bool,
    pub share_var_tbl:HashMap<String,VarType>
}
impl SB_State{
    pub fn new()->Self{
        return Self{
            cmd_buf:Vec::new(),
            cmd_pos:0,
            stack:Vec::new(),
            p_user_data:null_mut(),
            run_script:true,
            share_var_tbl:HashMap::new()
        };
    }
    pub fn load_from_file(file:&str)->Result<Self,String>{
        let file=RW_File::open_read(file);
        if let Ok(f)=file{
            let src_res=f.read_utf8_text();
            if let Ok(src_str)=src_res{
                let mut state=Self::new();
                
                let mut parser=SyntaxParser::new();
                parser.regist(String::from("result"),TypeInfo::Share(String::new()));
                parser.regist(String::from("load"),TypeInfo::NativeFunc(Some(load)));
                parser.regist(String::from("show"),TypeInfo::NativeFunc(Some(show)));
                parser.regist(String::from("text"),TypeInfo::NativeFunc(Some(text)));
                parser.regist(String::from("wait"),TypeInfo::NativeFunc(Some(wait)));
                parser.regist(String::from("create_anim_set"),
                    TypeInfo::NativeFunc(Some(create_anim_set)));
                parser.regist(String::from("selection"),
                    TypeInfo::NativeFunc(Some(selection)));
                parser.regist(String::from("left"),
                    TypeInfo::Const(VarType::Int(LEFT_IMAGE)));
                parser.regist(String::from("right"),
                    TypeInfo::Const(VarType::Int(RIGHT_IMAGE)));
                parser.regist(String::from("bg"),
                    TypeInfo::Const(VarType::Int(BG_IMAGE)));
                parser.regist(String::from("player"),
                    TypeInfo::Const(VarType::Int(PLAYER_CHARA_IMAGE)));

                let parse_res=parser.parse(&src_str,&mut state.cmd_buf);
                if let Ok(())=parse_res{
                
                    return Ok(state);
                }else{
                    return Err(parse_res.unwrap_err());
                }
            }else{
                return Ok(Self::new());
            }
        }else{
            return Err(file.unwrap_err());
        }
    

    }
    pub fn value_to_string(&self,idx:usize)->String{
        if self.stack.len()<=idx{
            return String::new();
        }
        if let VarType::Str(s)=&self.stack[self.stack.len()-idx-1]{
            return s.clone();
        }
        return String::new();
    }

    pub fn value_to_int(&self,idx:usize)->i32{
        if self.stack.len()<=idx{
            return 0;
        }
        if let VarType::Int(i)=self.stack[self.stack.len()-idx-1]{
            return i;
        }else if let VarType::Float(f)=self.stack[self.stack.len()-idx-1]{
            return f as i32;
        }
        return 0;

    }
    pub fn value_to_bool(&self,idx:usize)->bool{
        if self.stack.len()<=idx{
            return false;
        }
        if let VarType::Int(i)=self.stack[self.stack.len()-idx-1]{
            return i==1;
        }else if let VarType::Float(f)=self.stack[self.stack.len()-idx-1]{
            return (f as i32)==1;
        }else if let VarType::Bool(b)=self.stack[self.stack.len()-idx-1]{
            return b;
        }
        return false;

    }
    pub fn run(&mut self){
        while self.run_script && self.cmd_pos<self.cmd_buf.len(){
            match &self.cmd_buf[self.cmd_pos]{
                CmdType::Nop=>{},
                CmdType::PushStr(s)=>{
                    self.stack.push(VarType::Str(s.clone()));
                },
                CmdType::PushInt(i)=>{
                    self.stack.push(VarType::Int(*i));
                },
                CmdType::PushFloat(f)=>{
                    self.stack.push(VarType::Float(*f));
                },
                CmdType::PushBool(b)=>{
                    self.stack.push(VarType::Bool(*b));
                },
                CmdType::PushVar(idx)=>{
                    self.stack.push(self.stack[*idx].clone());
                },
                CmdType::PushShare(s)=>{
                    self.stack.push(self.share_var_tbl[s].clone());
                },
                CmdType::Add=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Int(i1+i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float((i1 as f32)+f2));
                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float(f1+f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Int((f1 as i32)+i));

                        }
                    }
                },
                CmdType::Sub=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Int(i1-i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float((i1 as f32)-f2));
                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float(f1-f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Int((f1 as i32)-i));

                        }
                    }
                },
                CmdType::Mul=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Int(i1*i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float((i1 as f32)*f2));
                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float(f1*f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Int((f1 as i32)*i));

                        }
                    }                    
                },
                CmdType::Div=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Int(i1/i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float((i1 as f32)/f2));
                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Float(f1/f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Int((f1 as i32)/i));

                        }
                    }                    
                },
                CmdType::Mod=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Int(i1%i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Int(i1%(f2 as i32)));
                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Int((f1 as i32)%(f2 as i32)));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Int((f1 as i32)%i));

                        }
                    }else if let VarType::Bool(b1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(b1 && 1.0==f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Bool(b1 && 1==i));

                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(b1 && b2));

                        }

                    }                    
                },
                CmdType::And=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Bool(1==i1 && 1==i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(1==i1 && 1.0==f2));
                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(1==i1 && b2));

                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 && 1.0==f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 && 1==i));

                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 && b2));

                        }
                    }else if let VarType::Bool(b1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(b1 && 1.0==f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Bool(b1 && 1==i));

                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(b1 && b2));

                        }

                    }
                },
                CmdType::Or=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::Int(i1)=op1{
                        if let VarType::Int(i2)=op2{
                            self.stack.push(VarType::Bool(1==i1 || 1==i2));
                        }else if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(1==i1 || 1.0==f2));
                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(1==i1 || b2));

                        }
                    }else if let VarType::Float(f1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 || 1.0==f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 || 1==i));

                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(1.0==f1 || b2));

                        }
                    }else if let VarType::Bool(b1)=op1{
                        if let VarType::Float(f2)=op2{
                            self.stack.push(VarType::Bool(b1 || 1.0==f2));
                        }else if let VarType::Int(i)=op2{
                            self.stack.push(VarType::Bool(b1 || 1==i));

                        }else if let VarType::Bool(b2)=op2{
                            self.stack.push(VarType::Bool(b1 || b2));

                        }

                    }

                },
                CmdType::Set(idx)=>{
                    let op=self.stack.pop().unwrap();
                    self.stack[*idx]=op;
                },
                CmdType::SetShare(name)=>{
                    let op=self.stack.pop().unwrap();
                    self.share_var_tbl.insert(name.clone(),op);
                },
                CmdType::JmpN(jmp_pos_op)=>{
                    if let Some(jmp_pos)=jmp_pos_op{
                        self.cmd_pos=*jmp_pos;
                    }else{
                        assert!(false);
                    }
                },
                CmdType::GenShare(name)=>{
                    self.share_var_tbl.insert(name.clone(),VarType::None);
                },
                CmdType::GenVar=>{
                    self.stack.push(VarType::None);
                },
                CmdType::Return=>{
                    let pos=self.stack.pop().unwrap();
                    if let VarType::Pos(idx)=pos{
                        self.cmd_pos=idx;
                    }
                },
                CmdType::Goto(jmp_pos_op)=>{
                    if let Some(jmp_pos)=jmp_pos_op{
                        self.cmd_pos=*jmp_pos;
                    }else{
                        assert!(false);
                    }
                },
                CmdType::Gosub(jmp_pos_op)=>{
                    if let Some(jmp_pos)=jmp_pos_op{
                        self.stack.push(VarType::Pos(self.cmd_pos));
                        self.cmd_pos=*jmp_pos;
                    }else{
                        assert!(false);
                    }
                },
                CmdType::Call(f,argn)=>{
                    let arg_num=*argn;
                    let ret_num=(*f)(self.p_user_data,self);
                    for i in 0..arg_num{
                        self.stack.remove(self.stack.len()-ret_num-1);
                    }
                },
            }
            self.cmd_pos=self.cmd_pos+1;
        }
    }
}