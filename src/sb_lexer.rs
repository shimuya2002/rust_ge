use std::str::*;
use std::ascii::*;
use std::fmt;
#[derive(PartialEq,Debug)]
pub enum TokenType{
    Ident(String),
    LitString(String),
    LitInt(i32),
    LitFloat(f32),
    Comma,
    LQuote,
    RQuote,
    Label(String),
    Goto,
    Gosub,
    Dim,
    As,
    Share,
    If,
    Then,
    Else,
    Elseif,
    Endif,
    While,
    Do,
    Wend,
    Equal,
    Notequal,
    G,
    GE,
    L,
    LE,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Return,
    Unknown(char)

}
impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Ident(s)=>{
                write!(f,"Token type ident {}",s)
            },
            Self::LitString(s)=>{
                write!(f,"Token type literal string \"{}\"",s)

            },
            Self::LitInt(i)=>{
                write!(f,"Token type literal int {}",i)

            },
            Self::LitFloat(fl)=>{
                write!(f,"Token type literal float {}",fl)

            },
            Self::Comma=>{
                write!(f,"Token type \",\"")

            },
            Self::LQuote=>{
                write!(f,"Token type \"(\"")

            },
            Self::RQuote=>{
                write!(f,"Token type \")\"",)

            },
            Self::Label(s)=>{
                write!(f,"Token type label #{}",s)

            },
            Self::Goto=>{
                write!(f,"Token type goto")

            },
            Self::Gosub=>{
                write!(f,"Token type gosub")

            },
            Self::Dim=>{
                write!(f,"Token type dim")
            },
            Self::As=>{
                write!(f,"Token type as")
            },
            Self::Share=>{
                write!(f,"Token type share")
            },
            Self::If=>{
                write!(f,"Token type if")
            },
            Self::Then=>{
                write!(f,"Token type then")
            },
            Self::Else=>{
                write!(f,"Token type else")
            },
            Self::Elseif=>{
                write!(f,"Token type elseif")
            },
            Self::Endif=>{
                write!(f,"Token type endif")
            },
            Self::While=>{
                write!(f,"Token type while")
            },
            Self::Do=>{
                write!(f,"Token type do")
            },
            Self::Wend=>{
                write!(f,"Token type wend")
            },
            Self::Equal=>{
                write!(f,"Token type \"==\"")

            },
            Self::Notequal=>{
                write!(f,"Token type \"<>\"")

            },
            Self::G=>{
                write!(f,"Token type \">\"")

            },
            Self::GE=>{
                write!(f,"Token type \">=\"")

            },
            Self::L=>{
                write!(f,"Token type \"<\"")
            },
            Self::LE=>{
                write!(f,"Token type \"<=\"")

            },
            Self::Assign=>{
                write!(f,"Token type \"=\"")

            },
            Self::Add=>{
                write!(f,"Token type \"+\"")

            },
            Self::Sub=>{
                write!(f,"Token type \"-\"")

            },
            Self::Mul=>{
                write!(f,"Token type \"*\"")

            },
            Self::Div=>{
                write!(f,"Token type \"/\"")

            },
            Self::Mod=>{
                write!(f,"Token type \"%\"")
               
            },
            Self::And=>{
                write!(f,"Token type \"and\"")

            },
            Self::Or=>{
                write!(f,"Token type \"or\"")

            },
            Self::Return=>{
                write!(f,"Token type return")

            },
            Self::Unknown(c)=>{
                write!(f,"Token type unknown \"{}\"",c)

            }
        }
    }
}
pub struct LexerParser{
    src:String,
    parse_pos:usize,
    ln_no:usize
}
impl LexerParser{
    pub fn new(src:String)->Self{
        
        return Self{
            src:src.clone(),
            parse_pos:0,
            ln_no:1
        };
    }
    pub fn peek(&mut self)->Option<TokenType>{
        let tmp_pos=self.parse_pos;
        let ln_no=self.ln_no;
        let r=self.get();
        self.parse_pos=tmp_pos;
        self.ln_no=self.ln_no;
        return r;
    }
    pub fn get(&mut self)->Option<TokenType>{
        while self.parse_pos<self.src.len() && 
            self.src.chars().nth(self.parse_pos).unwrap().is_whitespace(){
            self.parse_pos=self.parse_pos+1;
        }
        if self.parse_pos>=self.src.len(){
            return None;
        }
        let cur_c=self.src.chars().nth(self.parse_pos).unwrap();
        if cur_c.is_alphabetic(){
            let b_idx=self.parse_pos;
            self.parse_pos=self.parse_pos+1;
            while self.parse_pos<self.src.len() {
                let tmp=self.src.chars().nth(self.parse_pos).unwrap();
                if !(tmp.is_alphanumeric() || '_'==tmp){
                    break;
                }
                self.parse_pos=self.parse_pos+1;
                
            }
            let e_idx=self.parse_pos;
            let ident_str=self.src[b_idx..e_idx].to_string().to_lowercase();
            match ident_str.as_str(){
                "goto"=>{
                    return Some(TokenType::Goto);
                },
                "gosub"=>{
                    return Some(TokenType::Gosub);
                },
                "dim"=>{
                    return Some(TokenType::Dim);
                },
                "as"=>{
                    return Some(TokenType::As);
                },
                "share"=>{
                    return Some(TokenType::Share);
                },

                "if"=>{
                    return Some(TokenType::If);
                },
                "then"=>{
                    return Some(TokenType::Then);
                },
                "else"=>{
                    return Some(TokenType::Else);
                },
                "elseif"=>{
                    return Some(TokenType::Elseif);
                },
                "endif"=>{
                    return Some(TokenType::Endif);
                },
                "while"=>{
                    return Some(TokenType::While);
                },
                "do"=>{
                    return Some(TokenType::Do);
                },
                "wend"=>{
                    return Some(TokenType::Wend);
                },
                "and"=>{
                    return Some(TokenType::And);
                },
                "or"=>{
                    return Some(TokenType::Or);
                },
                "return"=>{
                    return Some(TokenType::Return);
                },

                _=>{
                    return Some(TokenType::Ident(ident_str));
                }
            }
        } else if cur_c.is_digit(10){
            let b_idx=self.parse_pos;
            self.parse_pos=self.parse_pos+1;
            while self.parse_pos<self.src.len() && 
                self.src.chars().nth(self.parse_pos).unwrap().is_digit(10){
                self.parse_pos=self.parse_pos+1;
            }
            let mut is_float=false;
            if self.parse_pos<self.src.len() && 
                '.'==self.src.chars().nth(self.parse_pos).unwrap(){
                is_float=true;
                self.parse_pos=self.parse_pos+1;
                while self.parse_pos<self.src.len() && 
                    self.src.chars().nth(self.parse_pos).unwrap().is_digit(10){
                    self.parse_pos=self.parse_pos+1;
                }
                
            }
            let e_idx=self.parse_pos;
            let lit_str=self.src[b_idx..e_idx].to_string();
            if is_float{
                return Some(TokenType::LitFloat(lit_str.parse().unwrap()));

            }else{
                return Some(TokenType::LitInt(lit_str.parse().unwrap()));

            }
        }else{
            match cur_c{
                '\"'=>{
                    self.parse_pos=self.parse_pos+1;
                    let b_idx=self.parse_pos;
                    while self.parse_pos<self.src.len() && 
                        cur_c!=self.src.chars().nth(self.parse_pos).unwrap(){
                        self.parse_pos=self.parse_pos+1;
                    }
                    let e_idx=self.parse_pos;
                    self.parse_pos=self.parse_pos+1;
                    let lit_str=self.src[b_idx..e_idx].to_string();
                    return Some(TokenType::LitString(lit_str));

                },
                '@'=>{
                    let b_idx=self.parse_pos;
                    self.parse_pos=self.parse_pos+1;
                    while self.parse_pos<self.src.len() && 
                        self.src.chars().nth(self.parse_pos).unwrap().is_alphanumeric(){
                        self.parse_pos=self.parse_pos+1;
                    }
                    let e_idx=self.parse_pos;
                    let label_str=self.src[b_idx..e_idx].to_string().to_lowercase();
                    return Some(TokenType::Label(label_str))
                },
                ','=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Comma);
                },
                '('=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::LQuote);
                },
                ')'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::RQuote);
                },
                '='=>{
                    self.parse_pos=self.parse_pos+1;
                    if self.parse_pos<self.src.len(){
                        let n_c=self.src.chars().nth(self.parse_pos).unwrap();
                        match n_c{
                            '>'=>{
                                self.parse_pos=self.parse_pos+1;
                                return Some(TokenType::GE);
                            },
                            '<'=>{
                                self.parse_pos=self.parse_pos+1;
                                return Some(TokenType::LE);
                            },
                            '='=>{
                                self.parse_pos=self.parse_pos+1;
                                return Some(TokenType::Equal);

                            },
                            _=>{}
                        }
                        
                    }
                    return Some(TokenType::Assign);
                    
                },
                '<'=>{
                    self.parse_pos=self.parse_pos+1;
                    if self.parse_pos<self.src.len() {

                        let n_c=self.src.chars().nth(self.parse_pos).unwrap();
                        if '>'==n_c{
                            self.parse_pos=self.parse_pos+1;
                            return Some(TokenType::Notequal);
                        }else if '='==n_c{
                            self.parse_pos=self.parse_pos+1;
                            return Some(TokenType::LE);

                        }else{}
                    }
                    return Some(TokenType::L);
                    
                },
                '>'=>{

                    self.parse_pos=self.parse_pos+1;
                    if self.parse_pos<self.src.len() {

                        let n_c=self.src.chars().nth(self.parse_pos).unwrap();
                        if '='==n_c{
                            self.parse_pos=self.parse_pos+1;
                            return Some(TokenType::GE);

                        }else{}
                    }              
                    return Some(TokenType::G);
                },
                '+'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Add);

                },
                '-'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Sub);

                },
                '*'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Mul);

                },
                '/'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Div);

                },
                '%'=>{
                    self.parse_pos=self.parse_pos+1;
                    return Some(TokenType::Mod);

                },
                _=>{
                    return Some(TokenType::Unknown(cur_c));
                }
            }
        }

    }
}
# [cfg(test)]
mod tests{
    use crate::sb_lexer::*;
    #[test]
    fn test_lexer(){
        assert_eq!(Some(TokenType::Ident("ident".to_string())),
            LexerParser::new("Ident".to_string()).get());
        assert_eq!(Some(TokenType::LitString("Ident".to_string())),
            LexerParser::new("\"Ident\"".to_string()).get());
        assert_eq!(Some(TokenType::LitInt(12)),
            LexerParser::new("12".to_string()).get());
        assert_eq!(Some(TokenType::LitFloat(12.01)),
            LexerParser::new("12.01".to_string()).get());
        assert_eq!(Some(TokenType::Comma),
            LexerParser::new(",".to_string()).get());
        assert_eq!(Some(TokenType::LQuote),
            LexerParser::new("(".to_string()).get());
        assert_eq!(Some(TokenType::RQuote),
            LexerParser::new(")".to_string()).get());
        assert_eq!(Some(TokenType::Label("@ident".to_string())),
            LexerParser::new("@Ident".to_string()).get());
        assert_eq!(Some(TokenType::Goto),
            LexerParser::new("goto".to_string()).get());
        assert_eq!(Some(TokenType::Gosub),
            LexerParser::new("gosub".to_string()).get());
        assert_eq!(Some(TokenType::Dim),
            LexerParser::new("dim".to_string()).get());
        assert_eq!(Some(TokenType::If),
            LexerParser::new("if".to_string()).get());
        assert_eq!(Some(TokenType::Then),
            LexerParser::new("then".to_string()).get());
        assert_eq!(Some(TokenType::Else),
            LexerParser::new("else".to_string()).get());
        assert_eq!(Some(TokenType::Elseif),
            LexerParser::new("elseif".to_string()).get());
        assert_eq!(Some(TokenType::Endif),
            LexerParser::new("endif".to_string()).get());
        assert_eq!(Some(TokenType::While),
            LexerParser::new("while".to_string()).get());
        assert_eq!(Some(TokenType::Do),
            LexerParser::new("do".to_string()).get());
        assert_eq!(Some(TokenType::Wend),
            LexerParser::new("wend".to_string()).get());
        assert_eq!(Some(TokenType::Equal),
            LexerParser::new("==".to_string()).get());
        assert_eq!(Some(TokenType::Notequal),
            LexerParser::new("<>".to_string()).get());
        assert_eq!(Some(TokenType::G),
            LexerParser::new(">".to_string()).get());
        assert_eq!(Some(TokenType::GE),
            LexerParser::new(">=".to_string()).get());
        assert_eq!(Some(TokenType::L),
            LexerParser::new("<".to_string()).get());
        assert_eq!(Some(TokenType::LE),
            LexerParser::new("<=".to_string()).get());
        assert_eq!(Some(TokenType::Assign),
            LexerParser::new("=".to_string()).get());


    }
}