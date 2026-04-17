///BASICの変数定義を解析する処理を記載

impl SyntaxParser{
    ///DIM文を解析する
    ///'lex' 解析する字句列
    ///'cmd' 解析後のコマンドを収める配列
    fn parse_dim(&mut self,lex:&mut LexerParser,cmd:&mut Vec<CmdType>)
        ->Result<(),String>{
        let tok=lex.get();
        if let Some(TokenType::Dim)=tok{

            if let Some(TokenType::Ident(name))=lex.get(){
                //共有変数の指定があるか？
                match lex.peek(){
                    //指定あり
                    Some(TokenType::As)=>{
                        _=lex.get();
                        let share_tok=lex.get();
                        if let Some(TokenType::Share)=share_tok{
                            if let None=self.find(&name){
                                self.regist(name.clone(),TypeInfo::Share(name.clone()));
                                cmd.push(CmdType::GenShare(name.clone()));
                                return Ok(());
                            }else{
                                return Err(Self::gen_already_defined(&name));
                            }
                                                                
                        }
                    },
                    None|
                    _=>{
                        if let None=self.find(&name){
                            self.regist(name,TypeInfo::Var(self.get_stack_base()+self.get_var_num()));
                            cmd.push(CmdType::GenVar);
                            return Ok(());
                        }else{
                            return Err(format!("Variable {} redefined",name));
                        }

                    }
                }
                
            }
        }
        return Err("Invalid eos occurred!!".to_string());

            
    }

}