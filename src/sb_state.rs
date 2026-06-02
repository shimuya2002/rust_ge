use std::ffi::*;
use std::ptr::*;
use std::collections::*;
use crate::config::*;
use crate::sb_cmdtype::*;
use crate::sb_syntax::*;
use crate::rw_file::*;
//use crate::script_func::*;
pub struct SB_State{
    cmd_buf:Vec<CmdType>,
    cmd_pos:usize,
    pub stack:Vec<VarType>,
    pub p_user_data:*mut c_void,
    pub run_script:bool,
    pub share_var_tbl:HashMap<String,VarType>,
    pub is_error_raised:Option<String>
}
impl SB_State{
    pub fn new()->Self{
        return Self{
            cmd_buf:Vec::new(),
            cmd_pos:0,
            stack:Vec::new(),
            p_user_data:null_mut(),
            run_script:true,
            share_var_tbl:HashMap::new(),
            is_error_raised:None,
        };
    }
    pub fn load_from_file(file:&str,regist_tbl:Option<&HashMap<String,TypeInfo>>)->Result<Self,String>{
        let file=RW_File::open_read(file);
        if let Ok(f)=file{
            let src_res=f.read_utf8_text();
            if let Ok(src_str)=src_res{
                let mut state=Self::new();
                
                let mut parser=SyntaxParser::new();
                if let Some(tbl)=regist_tbl{
                    for k in tbl.keys(){
                        parser.regist(k.clone(),tbl[k].clone());
                    }

                }
               

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
        let req_idx=self.stack.len()-idx-1;
        let value=self.get_value(self.stack[req_idx].clone());
        if let VarType::Str(s)=value{
            return s.clone();
        }else if let VarType::Int(n)=value{
            return n.to_string();
        }else{
            return format!("Invalid value {}",value);
        }
        return String::new();
    }

    pub fn value_to_int(&self,idx:usize)->i32{
        if self.stack.len()<=idx{
            return 0;
        }
        let req_idx=self.stack.len()-idx-1;
        let value=self.get_value(self.stack[req_idx].clone());
        if let VarType::Int(i)=value{
            return i;
        }else if let VarType::Float(f)=value{
            return f as i32;
        }
        return 0;

    }
    pub fn value_to_bool(&self,idx:usize)->bool{
        if self.stack.len()<=idx{
            return false;
        }
        let req_idx=self.stack.len()-idx-1;
        let value=self.get_value(self.stack[req_idx].clone());
        if let VarType::Int(i)=value{
            return i==1;
        }else if let VarType::Float(f)=value{
            return (f as i32)==1;
        }else if let VarType::Bool(b)=value{
            return b;
        }
        return false;

    }
    fn add(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float(f1+f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Float(f1+(i as f32));
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.add(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.add(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Int(i1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float((i1 as f32)+f2);
            }else if let VarType::Int(i2)=op2{
                return VarType::Int(i1+i2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.add(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.add(op1,self.share_var_tbl[&op2_key].clone());
            }

        }else if let VarType::VarPos(op1_idx)=op1{
            return self.add(self.stack[op1_idx].clone(),op2);
        }else if let VarType::SharePos(op1_key)=op1{
            return self.add(self.share_var_tbl[&op1_key].clone(),op2);
        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    fn sub(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float(f1-f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Float(f1-(i as f32));
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.add(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.add(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Int(i1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float((i1 as f32)-f2);
            }else if let VarType::Int(i2)=op2{
                return VarType::Int(i1-i2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.sub(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.sub(op1,self.share_var_tbl[&op2_key].clone());
            }

        }else if let VarType::VarPos(op1_idx)=op1{
            return self.sub(self.stack[op1_idx].clone(),op2);
        }else if let VarType::SharePos(op1_key)=op1{
            return self.sub(self.share_var_tbl[&op1_key].clone(),op2);
        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    fn mul(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float(f1*f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Float(f1*(i as f32));
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.mul(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.mul(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Int(i1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float((i1 as f32)*f2);
            }else if let VarType::Int(i2)=op2{
                return VarType::Int(i1*i2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.mul(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.mul(op1,self.share_var_tbl[&op2_key].clone());
            }

        }else if let VarType::VarPos(op1_idx)=op1{
            return self.mul(self.stack[op1_idx].clone(),op2);
        }else if let VarType::SharePos(op1_key)=op1{
            return self.mul(self.share_var_tbl[&op1_key].clone(),op2);
        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    fn div(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float(f1/f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Float(f1/(i as f32));
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.div(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.div(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Int(i1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float((i1 as f32)/f2);
            }else if let VarType::Int(i2)=op2{
                return VarType::Int(i1/i2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.div(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.div(op1,self.share_var_tbl[&op2_key].clone());
            }

        }else if let VarType::VarPos(op1_idx)=op1{
            return self.div(self.stack[op1_idx].clone(),op2);
        }else if let VarType::SharePos(op1_key)=op1{
            return self.div(self.share_var_tbl[&op1_key].clone(),op2);
        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    fn r#mod(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float(f1%f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Float(f1%(i as f32));
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.r#mod(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.r#mod(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Int(i1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Float((i1 as f32)%f2);
            }else if let VarType::Int(i2)=op2{
                return VarType::Int(i1%i2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.r#mod(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.r#mod(op1,self.share_var_tbl[&op2_key].clone());
            }

        }else if let VarType::VarPos(op1_idx)=op1{
            return self.r#mod(self.stack[op1_idx].clone(),op2);
        }else if let VarType::SharePos(op1_key)=op1{
            return self.r#mod(self.share_var_tbl[&op1_key].clone(),op2);
        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    pub fn and(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Int(i1)=op1{
            if let VarType::Int(i2)=op2{
                return VarType::Bool(1==i1 && 1==i2);
            }else if let VarType::Float(f2)=op2{
                return VarType::Bool(1==i1 && 1.0==f2);
            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(1==i1 && b2);
            }else if let VarType::VarPos(op2_idx)=op2{
                return self.and(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.and(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Bool(1.0==f1 && 1.0==f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Bool(1.0==f1 && 1==i);

            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(1.0==f1 && b2);

            }else if let VarType::VarPos(op2_idx)=op2{
                return self.and(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.and(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Bool(b1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Bool(b1 && 1.0==f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Bool(b1 && 1==i);

            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(b1 && b2);

            }else if let VarType::VarPos(op2_idx)=op2{
                return self.and(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.and(op1,self.share_var_tbl[&op2_key].clone());
            }

        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);
    }
    pub fn or(&self,op1:VarType,op2:VarType)->VarType{
        if let VarType::Int(i1)=op1{
            if let VarType::Int(i2)=op2{
                return VarType::Bool(1==i1 || 1==i2);
            }else if let VarType::Float(f2)=op2{
                return VarType::Bool(1==i1 || 1.0==f2);
            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(1==i1 || b2);

            }else if let VarType::VarPos(op2_idx)=op2{
                return self.or(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.or(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Float(f1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Bool(1.0==f1 || 1.0==f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Bool(1.0==f1 || 1==i);

            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(1.0==f1 || b2);

            }else if let VarType::VarPos(op2_idx)=op2{
                return self.or(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.or(op1,self.share_var_tbl[&op2_key].clone());
            }
        }else if let VarType::Bool(b1)=op1{
            if let VarType::Float(f2)=op2{
                return VarType::Bool(b1 || 1.0==f2);
            }else if let VarType::Int(i)=op2{
                return VarType::Bool(b1 || 1==i);

            }else if let VarType::Bool(b2)=op2{
                return VarType::Bool(b1 || b2);

            }else if let VarType::VarPos(op2_idx)=op2{
                return self.or(op1,self.stack[op2_idx].clone());
            }else if let VarType::SharePos(op2_key)=op2{
                return self.or(op1,self.share_var_tbl[&op2_key].clone());
            }

        }
        println!("Invalid value {}",op1);
        return VarType::Int(0);       
    }
    pub fn get_value(&self,op:VarType)->VarType{
        if let VarType::VarPos(idx)=op{
            return self.stack[idx].clone();
        }else if let VarType::SharePos(idx_key)=op{
            return self.share_var_tbl[&idx_key].clone();
        }
        return op;
    }
    pub fn dump(&self){
        for i in 0..self.cmd_buf.len(){
            println!("{}:{}",i,self.cmd_buf[i]);
        }
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
                    self.stack.push(VarType::VarPos(*idx));
                },
                CmdType::PushShare(s)=>{
                    self.stack.push(VarType::SharePos(s.clone()));
                },
                CmdType::Add=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.add(op1,op2));
                },
                CmdType::Sub=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.sub(op1,op2));
                },
                CmdType::Mul=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.mul(op1,op2));
                },
                CmdType::Div=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.div(op1,op2));             
                },
                CmdType::Mod=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.r#mod(op1,op2));                 
                },
                CmdType::And=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.and(op1,op2));   
                },
                CmdType::Or=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    self.stack.push(self.or(op1,op2));   
                },
                CmdType::Set=>{
                    let op2=self.stack.pop().unwrap();
                    let op1=self.stack.pop().unwrap();
                    if let VarType::VarPos(dst_idx)=op1{
                        self.stack[dst_idx]=self.get_value(op2);

                    }else if let VarType::SharePos(idx_key)=op1{
                        self.share_var_tbl.insert(idx_key,self.get_value(op2));

                    }
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
                    if let VarType::JmpPos(idx)=pos{
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
                        self.stack.push(VarType::JmpPos(self.cmd_pos));
                        self.cmd_pos=*jmp_pos;
                    }else{
                        assert!(false);
                    }
                },
                CmdType::Call(f,argn)=>{
                    let arg_num=*argn;
                    let ret=(*f)(self.p_user_data,self,arg_num as i32);
                    /*for i in 0..self.stack.len(){
                        println!("{}",self.stack[i]);
                    }*/
                    for i in 0..arg_num{
                        self.stack.pop();
                    }
                    if let Some(v)=ret{
                        self.stack.push(v);
                    }else{
                        self.stack.push(VarType::None);
                    }
                    //println!("Stack len is {}",self.stack.len());
                },
                CmdType::Pop=>{
                    self.stack.pop();
                }
            }
            self.cmd_pos=self.cmd_pos+1;
        }
    }
}