impl SyntaxParser{

    fn parse_expr(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
            return self.parse_bool_expr(lex,cmd);
    
    }
    fn parse_bool_expr(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        
        
        let op1=self.parse_add_sub_expr(lex,cmd);
        let mut b_float=false;
        if let Err(_)=op1{
            return op1;
        }
        while let Some(n_tok)=lex.peek(){
            let mut c=CmdType::Nop;
            match n_tok{
                TokenType::And=>{
                    c=CmdType::And;
                },
                TokenType::Or=>{
                    c=CmdType::Or;
                },
                _=>{
                    break;
                }
            }
            _=lex.get();
            let op2=self.parse_mul_div_expr(lex,cmd);
            if let Err(_)=op2{
                return op2;
            }
            cmd.push(c);
            
        }
        return Ok(());


    }
    fn parse_add_sub_expr(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        
        let op1=self.parse_mul_div_expr(lex,cmd);
        if let Err(_)=op1{
            return op1;
        }
        while let Some(n_tok)=lex.peek(){
            let mut c=CmdType::Nop;
            match n_tok{
                TokenType::Add=>{
                    c=CmdType::Add;
                },
                TokenType::Sub=>{
                    c=CmdType::Sub;
                },
                _=>{
                    break;
                }
            }
            _=lex.get();
            let op2=self.parse_mul_div_expr(lex,cmd);
            if let Err(_)=op2{
                return op2;
            }
            cmd.push(c);
        }
        return Ok(());
    }
    fn parse_mul_div_expr(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{

        let op1=self.parse_value(lex,cmd);
        if let  Err(_)=op1{
            return op1;
        }
        while let Some(n_tok)=lex.peek(){
            let mut c=CmdType::Nop;
            match n_tok{
                TokenType::Mul=>{
                    c=CmdType::Mul;
                },
                TokenType::Div=>{
                    c=CmdType::Div;
                },
                TokenType::Mod=>{
                    c=CmdType::Mod;
                },
                _=>{
                    break;
                }
            }
            _=lex.get();
            let op2=self.parse_value(lex,cmd);
            if let Err(_)=op2{
                return op2;
            }
            cmd.push(c);
            
        }
        return Ok(());
    }
    fn parse_value(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        if let Some(cur_tok)=lex.get(){
            match cur_tok{
                TokenType::LitString(s)=>{
                    cmd.push(CmdType::PushStr(s.clone()));
                    return Ok(());
                },
                TokenType::LitInt(n)=>{
                    cmd.push(CmdType::PushInt(n));
                    return Ok(());
                },
                TokenType::LitFloat(f)=>{
                    cmd.push(CmdType::PushFloat(f));
                    return Ok(());
                }
                TokenType::Ident(n)=>{
                    if let Some(mut t)=self.find(n.as_str()) {
                        let def_ti:&RefCell<TypeInfo>=Rc::borrow(&t);
                        match def_ti.borrow().deref(){
                            TypeInfo::Var(idx)=>{
                                cmd.push(CmdType::PushVar(*idx));
                                return Ok(());
                            },
                            TypeInfo::Const(v)=>{
                                match v{
                                    VarType::Int(vn)=>{
                                        cmd.push(CmdType::PushInt(*vn));
                                        return Ok(());
                                    },
                                    VarType::Float(vf)=>{
                                        cmd.push(CmdType::PushFloat(*vf));
                                        return Ok(());
                                    },
                                    VarType::Str(vs)=>{
                                        cmd.push(CmdType::PushStr(vs.clone()));
                                        return Ok(());

                                    },
                                    VarType::Bool(b)=>{
                                        cmd.push(CmdType::PushBool(*b));
                                        return Ok(());


                                    },
                                    _=>{
                                        assert!(false);
                                    }
                                }
                            },
                            _=>{

                            }
                        }
                    }
                    return Err(format!("Unknown variable {} found.",n));
                },
                TokenType::LQuote=>{
                    let r=self.parse_expr(lex,cmd);
                    if let Some(TokenType::RQuote)=lex.peek(){
                        return Ok(());
                    }else{
                        return Err("Needs ')'".to_string());
                    }
                }
                _=>{
                    return Err(format!("Invalid token occurred {}",cur_tok));

                }
            }

        }else{
            return Err("Invalid end of string occurred!".to_string());
        }
    }

}