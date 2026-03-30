use std::collections::*;
use std::rc::*;
use std::ffi::*;
use std::ptr::*;
use std::cell::*;
use std::borrow::*;
use std::ops::*;
use std::fmt;
use crate::sb_cmdtype::*;
use crate::sb_lexer::*;

pub enum TypeInfo{
    Var(usize),
    Share(String),
    Const(VarType),
    Label(Option<usize>),
    NativeFunc(Option<NativeFuncType>)
    
}
impl fmt::Display for TypeInfo{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Var(s)=>{
                write!(f,"Type variable {}",s)//Rc::get_mut(&mut ti).unwrap())
            },
            Self::Share(s)=>{
                write!(f,"Type share variable {}",s)//Rc::get_mut(&mut ti).unwrap())

            },
            Self::Const(t)=>{
                write!(f,"Const {}",t)

            },
            Self::Label(_)=>{
                write!(f,"Label")

            },
            Self::NativeFunc(_)=>{
                write!(f,"Native function")
            }

        }
    }
}
//#[derive(Debug)]
struct Namespace{
    pub stack_base:usize,
    pub tbl:HashMap<String,Rc<RefCell<TypeInfo>>>,
    pub var_num:usize
}
impl Namespace{
    pub fn new(stack_base:usize)->Self{
        return Self{
            stack_base:stack_base,
            tbl:HashMap::new(),
            var_num:0
        };
    }
}
struct LabelPosInfo{
    pub ref_tbl:Vec<usize>,
    pub label_pos:Option<usize>,
}
pub struct SyntaxParser{
    ns_stack:Vec<Namespace>,
    label_ref_tbl:HashMap<String,LabelPosInfo>
}
impl SyntaxParser{
    pub fn new()->Self{
        let mut ns=Namespace::new(0);
        ns.tbl.insert("true".to_string(),
                        Rc::new(RefCell::new(TypeInfo::Const(VarType::Bool(true)))));
        ns.tbl.insert("false".to_string(),
                        Rc::new(RefCell::new(TypeInfo::Const(VarType::Bool(false)))));
        return Self{
            ns_stack:vec![ns],
            label_ref_tbl:HashMap::new(),
        };
    }

    pub fn regist(&mut self,name:String,ti:TypeInfo){
        assert!(0<self.ns_stack.len());
        let mut top_item=self.ns_stack.last_mut().unwrap();
        if let TypeInfo::Var(_)=ti{

            let v_num=top_item.var_num;
            top_item.var_num=v_num+1;

        }
        top_item.tbl.insert(name,Rc::new(RefCell::new(ti)));
    }
    fn get_var_num(&self)->usize{
        assert!(0<self.ns_stack.len());
        let mut top_item=self.ns_stack.last();
        return top_item.unwrap().var_num;
    }
    fn get_stack_base(&self)->usize{
        assert!(0<self.ns_stack.len());
        let mut top_item=self.ns_stack.last();
        return top_item.unwrap().stack_base;
    }

    fn find(&self,name:&str)->Option<Rc<RefCell<TypeInfo>>>{
        for i in self.ns_stack.len()-1..=0{
            if self.ns_stack[i].tbl.contains_key(name){
                return Some(self.ns_stack[i].tbl[name].clone());
            }
        }
        return None;
    }

    fn gen_invalid_eos(file:&str,line:u32)->String{
        return format!("{}:{}Invalid eos occurred!",file,line);
    }
    fn gen_invalid_token(tok:TokenType)->String{
        return format!("Invalid token {} found!",tok);
    }
    fn gen_need_token(required:TokenType,occurred:TokenType)->String{
        return format!("Needs \"{}\" not {}",required,occurred);
    }
    fn gen_need_then(tok:TokenType)->String{
        return format!("Needs \"then\" not {}",tok);
    }
    fn gen_already_defined(name:&String)->String{
        return format!("Ident {} redefined",name);
    }
    fn gen_not_defined(name:&String)->String{
        return format!("Ident {} not defined",name);
        
    }
    fn gen_left_must_be_variable(tinfo:&TypeInfo)->String{
        return format!("Assign left hand must be a variable.Not {}",tinfo);
    }
    fn gen_invalid_type(required:TypeInfo,occurred:&TypeInfo)->String{
        return format!("Must be {}.Not {}",required,occurred)
    }
    fn gen_label_not_defined(name:String)->String{
        return format!("Label {} not defined.",name);
    }
    fn add_label_ref(&mut self,label_name:&String,pos:usize){
        if !self.label_ref_tbl.contains_key(label_name){
            self.label_ref_tbl.insert(label_name.clone(),
                LabelPosInfo{ref_tbl:Vec::new(),label_pos:None});

        }
        self.label_ref_tbl.get_mut(label_name).unwrap().ref_tbl.push(pos);

    }
    fn set_label_pos(&mut self,label_name:&String,pos:usize){
        if !self.label_ref_tbl.contains_key(label_name){
            self.label_ref_tbl.insert(label_name.clone(),
                LabelPosInfo{ref_tbl:Vec::new(),label_pos:Some(pos)});

        }else{
            self.label_ref_tbl.get_mut(label_name).unwrap().label_pos=Some(pos);
        }
    }
    fn gen_jmp_label(pos:usize)->String{
        return format!("Jmp_Label_{}",pos);
    }
}
include!("./sb_dim_syntax.rs");
include!("./sb_exp_syntax.rs");
include!("./sb_if_syntax.rs");
include!("./sb_while_syntax.rs");
include!("./sb_etc_syntax.rs");

# [cfg(test)]
mod tests{
    use crate::sb_lexer::*;
    use crate::sb_cmdtype::*;
    use crate::sb_syntax::*;
    #[test]
    fn test_syntax(){
        let mut cmd_buf=Vec::new();
        let dim_r=SyntaxParser::new().parse(&"Dim a".to_string(),&mut cmd_buf);
        assert_eq!(Ok(()),dim_r);
        assert_eq!(CmdType::GenVar,cmd_buf[0]);

        cmd_buf=Vec::new();
        let calc_r=SyntaxParser::new().parse(&"Dim a\na=1+1".to_string(),&mut cmd_buf);
        assert_eq!(Ok(()),calc_r);
        assert_eq!(CmdType::GenVar,cmd_buf[0]);
        assert_eq!(CmdType::PushVar(0),cmd_buf[1]);
        assert_eq!(CmdType::PushInt(1),cmd_buf[2]);
        assert_eq!(CmdType::PushInt(1),cmd_buf[3]);
        assert_eq!(CmdType::Add,cmd_buf[4]);

        cmd_buf=Vec::new();
        let if_r=SyntaxParser::new().parse(&"if true then\nendif".to_string(),&mut cmd_buf);
        assert_eq!(Ok(()),if_r);
        assert_eq!(CmdType::PushBool(true),cmd_buf[0]);
        assert_eq!(CmdType::JmpN(Some(2)),cmd_buf[1]);

        cmd_buf=Vec::new();
        let while_r=SyntaxParser::new().parse(&"while true do\nwend".to_string(),&mut cmd_buf);
        assert_eq!(Ok(()),while_r);
        assert_eq!(CmdType::PushBool(true),cmd_buf[0]);
        assert_eq!(CmdType::JmpN(Some(3)),cmd_buf[1]);
        assert_eq!(CmdType::Goto(Some(0)),cmd_buf[2]);
    }
}