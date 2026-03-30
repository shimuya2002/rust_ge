impl SyntaxParser{
    fn parse_while(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        let tok=lex.get();
        if let Some(ttype)=tok{
            let cont_label=Self::gen_jmp_label(cmd.len());
            self.set_label_pos(&cont_label,cmd.len());
            if let TokenType::While=ttype{
                let expr_r=self.parse_expr(lex,cmd);
                if let Ok(())=expr_r{
                    if let Some(do_ttype)=lex.get(){
                        let jmp_label=Self::gen_jmp_label(cmd.len());
                        self.add_label_ref(&jmp_label,cmd.len());
                        cmd.push(CmdType::JmpN(None));
                        if let TokenType::Do=do_ttype{
                            let chunck_r=self.parse_chunck(lex,cmd);
                            if let Ok(())=chunck_r{
                                if let Some(wend_ttype)=lex.get(){
                                    if let TokenType::Wend=wend_ttype{
                                        self.add_label_ref(&cont_label,cmd.len());
                                        cmd.push(CmdType::Goto(None));
                                        self.set_label_pos(&jmp_label,cmd.len());
                                        return Ok(());

                                    }else{
                                        return Err(Self::gen_need_token(TokenType::Wend,wend_ttype));
                                    }
                                }else{
                                    return Err(Self::gen_invalid_eos(file!(),line!()));

                                }
                            }else{
                                return chunck_r;
                            }
                        }else{
                            return Err(Self::gen_need_token(TokenType::Do,do_ttype));
                        }    
                    }else{
                        return Err(Self::gen_invalid_eos(file!(),line!()));
                    }
                }else{
                    return expr_r;
                }
            }else{
                return Err(Self::gen_need_token(TokenType::While,ttype));
            }
        }else{
            return Err(Self::gen_invalid_eos(file!(),line!()));
        }
    }
}