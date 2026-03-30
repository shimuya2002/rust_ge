impl SyntaxParser{
    fn parse_goto(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        let tok=lex.get();
        if let Some(goto_ttype)=tok{
            match goto_ttype{
                TokenType::Goto|
                TokenType::Gosub=>{
                    if let Some(ident_ttype)=lex.get(){
                        if let TokenType::Label(label_name)=ident_ttype{
                            if let None=self.find(&label_name){
                                self.add_label_ref(&label_name,cmd.len());
                            }else{
                                return Err(Self::gen_already_defined(&label_name));
                            }

                        }else{
                            return Err(Self::gen_need_token(TokenType::Ident("".to_string()),ident_ttype));
                        }
                    }else{
                        return Err(Self::gen_invalid_eos(file!(),line!()));
                    }

                },
                _=>{
                    return Err(Self::gen_need_token(TokenType::Goto,goto_ttype));

                }
            }
            if let TokenType::Gosub=goto_ttype{
                cmd.push(CmdType::Gosub(None));
            }else{
                cmd.push(CmdType::Goto(None));
            }
            return Ok(());
        }else{
            return Err(Self::gen_invalid_eos(file!(),line!()));
        }
    }

    pub fn parse(&mut self,src:&String,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        let mut lex=LexerParser::new(src.clone());
        let chunck_r= self.parse_chunck(&mut lex,cmd);
        if let Ok(())=chunck_r{
            cmd.push(CmdType::Nop);
            return self.resolve_label_ref(cmd);
        }else{
            return chunck_r;

        }
    }
    fn parse_chunck(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        while let Some(tok)=lex.peek(){
            match tok{
                TokenType::Elseif | 
                TokenType::Wend   |
                TokenType::Else   |
                TokenType::Endif=>{
                    return Ok(());
                },
                TokenType::Ident(name)=>{
                    lex.get();
                    if let Some(mut ti)=self.find(&name){
                        let assign_or_lq=lex.peek();
                        if let Some(TokenType::Assign)=assign_or_lq{
                            lex.get();
                            let left_type:&RefCell<TypeInfo>=Rc::borrow(&ti);
                            let left_ti_ref=left_type.borrow();
                            let left_ti=left_ti_ref.deref();
                            match left_ti{
                                TypeInfo::Var(idx)=>{
                                    cmd.push(CmdType::PushVar(*idx));
                                },
                                TypeInfo::Share(name)=>{
                                    cmd.push(CmdType::PushShare(name.clone()));
                                },
                                _=>{
                                    return Err(Self::gen_left_must_be_variable(left_ti));
                                }
                            }
                            let exp_r=self.parse_expr(lex,cmd);
                            if let Ok(())=exp_r{
                            }else{
                                return exp_r;
                            }
                        }else if let Some(TokenType::LQuote)=assign_or_lq{
                            let call_r=self.parse_call(&name,lex,cmd);
                            if let Ok(())=call_r{

                            }else{
                                return call_r;
                            }
                        }else{
                            return Err(Self::gen_invalid_eos(file!(),line!()));
                        }

                    }else{
                        return Err(Self::gen_already_defined(&name));
                    }
                },
                TokenType::If=>{
                    return self.parse_if(lex,cmd);
                },
                TokenType::While=>{
                    return self.parse_while(lex,cmd);
                },
                TokenType::Goto|
                TokenType::Gosub=>{
                    let r=self.parse_goto(lex,cmd);
                    if let Err(_)=r{
                        return r;
                    }
                    self.ns_stack.push(Namespace::new(self.get_stack_base()+self.get_var_num()+1));
                },
                TokenType::Dim=>{
                    let r=self.parse_dim(lex,cmd);
                    if let Err(_)=r{
                        return r;
                    }
                },
                TokenType::Return=>{
                    cmd.push(CmdType::Return);
                    if 0 < self.ns_stack.len(){
                        self.ns_stack.pop();

                    }
                },
                _=>{
                    return Err(Self::gen_invalid_token(tok));
                }
            }
        }
        return Ok(());
    }
    fn parse_call(&mut self,name:&String,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        if let Some(mut ti)=self.find(name){
            
            let ti_ref:&RefCell<TypeInfo>=Rc::borrow(&ti);
            let tinfo_ref=ti_ref.borrow();
            let tinfo=tinfo_ref.deref();
            if let TypeInfo::NativeFunc(f)=tinfo{
                let lq_type=lex.get();
                if let Some(lq_tok)=lq_type{
                    if let TokenType::LQuote=lq_tok{
                        let mut arg_tok=lex.peek();
                        let mut arg_num=0;
                        loop{
                            if let Some(TokenType::RQuote)=arg_tok{
                                break;
                            }
                            arg_num=arg_num+1;
                            let arg_r=self.parse_expr(lex,cmd);
                            if let Err(_)=arg_r{
                                return arg_r;
                            }
                            arg_tok=lex.get();
                            if let Some(TokenType::Comma)=arg_tok{
                                continue;
                            }else if let Some(TokenType::RQuote)=arg_tok{
                                continue;
                            }else{
                                return Err(Self::gen_invalid_eos(file!(),line!()));
                            }

                        }
                        cmd.push(CmdType::Call(f.unwrap(),arg_num));
                        return Ok(());
                    }else{
                        return Err(Self::gen_need_token(TokenType::LQuote,lq_tok));

                    }

                }else{
                    return Err(Self::gen_invalid_eos(file!(),line!()));
                }
            }else{
                return Err(
                        Self::gen_invalid_type(TypeInfo::NativeFunc(None),
                            tinfo));
            }
        }else{
            return Err(Self::gen_not_defined(&name));
        }
    }
    fn resolve_label_ref(&mut self,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        for k in self.label_ref_tbl.keys(){
            let mut ref_info=&self.label_ref_tbl[k];
            if let Some(label_pos)=ref_info.label_pos{
                for i in &ref_info.ref_tbl{
                    let mut cmd_type=cmd.get_mut(*i).unwrap();
                    match cmd_type{
                        CmdType::JmpN(_)=>{
                            *cmd_type=CmdType::JmpN(Some(label_pos));
                        },
                        CmdType::Goto(_)=>{
                            *cmd_type=CmdType::Goto(Some(label_pos));
                        },
                        CmdType::Gosub(_)=>{
                            *cmd_type=CmdType::Gosub(Some(label_pos));
                        },
                        _=>{
                            assert!(false);
                        }

                    }
                   // cmd[i]=cmd_type;
                }

            }else{
                return Err(Self::gen_label_not_defined(k.clone()));
            }

        }
        return Ok(());
    }
}