impl SyntaxParser{
    fn parse_if(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        let tok=lex.get();
        if let Some(ttype)=tok{
            match ttype{
                TokenType::If|
                TokenType::Elseif=>{
                    let expr_r=self.parse_expr(lex,cmd);
                    if let Ok(())=expr_r{
                        let jmp_label=Self::gen_jmp_label(cmd.len());
                        self.add_label_ref(&jmp_label,cmd.len());
                        cmd.push(CmdType::JmpN(None));
                        let then_tok=lex.get();
                        if let Some(then_ttype)=then_tok{
                            if let TokenType::Then=then_ttype{
                                let chunck_r=self.parse_chunck(lex,cmd);
                                if let Ok(())=chunck_r{
                                    self.set_label_pos(&jmp_label,cmd.len());
                                    if let Some(TokenType::Endif)=lex.peek(){
                                        return Ok(());
                                    }else{
                                        return self.parse_if(lex,cmd);
                                    }

                            
                                }else{
                                    return chunck_r;
                                }
                            }else{
                                return Err(Self::gen_need_token(TokenType::Then,then_ttype));
                            }
                        }else{
                            return Err(Self::gen_invalid_eos(file!(),line!()));
                        }
                    }else{
                        return expr_r;
                    }
                },
                TokenType::Else=>{
                    let chunck_r=self.parse_chunck(lex,cmd);
                    if let Ok(())=chunck_r{
                        let endif_tok=lex.peek();
                        if let Some(endif_ttype)=endif_tok{
                            if let TokenType::Endif=endif_ttype{

                                return Ok(());
                            }else{
                                return Err(Self::gen_need_token(TokenType::Endif, endif_ttype));
                            }
                        }else{
                            return Err(Self::gen_invalid_eos(file!(),line!()));
                        }
                    }else{
                        return chunck_r;
                    }
                },
                _=>{
                    return Err(Self::gen_invalid_token(ttype));

                }
            }
        }else{
            return Err(Self::gen_invalid_eos(file!(),line!()));
        }            
    }
}