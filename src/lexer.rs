use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::PRINT_;

#[derive(Debug, Clone, PartialEq)]
pub struct Token{
    pub value: String,
    pub value_type: TokenType
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType{
    Number,
    BinOp,
    Identifier,

    Let_k,
    Nil_k,
    fn_struct_k,
    ano_obj_k,
    Bool_true_t,
    Bool_false_t,

    Flag(Flags),

    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Semicolon,
    Colon,
    Comma,
    EOF,
    RightCurly,
    LeftCurly,
    Dot,
    RetType,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Flags{
    Assign_f,
    Const_f,
    Struct_f(Attr),
    Complex_f(Attr),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Attr{
    //Types
    Numeric,
    Bool,
    Object,
    Complex(String),
    ComplexKind,
    FnStruct,
}

static keywords: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("|", TokenType::Let_k);
    map.insert("!?", TokenType::Nil_k);
    map.insert("fn", TokenType::fn_struct_k);
    map.insert("obj", TokenType::ano_obj_k);
    map.insert("true", TokenType::Bool_true_t);
    map.insert("false", TokenType::Bool_false_t);
    map.insert("@", TokenType::RetType);
    map
});

pub unsafe fn get_flag(flag: &str, attr: Option<Attr>) -> Option<Flags> {
   match flag {
        "<asg>" => {
            Some(Flags::Assign_f)
        },
        "<const>" => {
            Some(Flags::Const_f)
        },
        "<structure>" => {
            let unwrap = attr.unwrap_or_else(||{
                panic!("Missing Attr")
            });
            Some(Flags::Struct_f(unwrap.clone()))
        },        
        "<complex>" => {
            let unwrap = attr.unwrap_or_else(||{
                panic!("Missing Attr")
            });
            Some(Flags::Complex_f(unwrap.clone()))
        },
        _ => {
            panic!("Unrecognised Flag")
        }
   } 
}

pub unsafe fn get_attr(atr: Option<&str>) -> Option<Attr> {
    if let Some(attr) = atr {
        match attr {
            "numeric" => {
                Some(Attr::Numeric)
            },
            "bool" => {
                Some(Attr::Bool)
            },
            "object" => {
                Some(Attr::Object)
            },
            "function" => {
                Some(Attr::FnStruct)
            },
            "complex" => {
                Some(Attr::ComplexKind)
            },
            _ => {
                Some(Attr::Complex(attr.to_string()))
            }
        } 
    }else{
        return None
    }
}

pub unsafe fn tokenize(src: String) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];
    let mut source: Vec<String> = src.chars().map(|x| x.to_string()).collect();

    while source.len() > 0{
        match source[0].as_str() {
            "(" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::LeftParen});
            },
            ";" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::Semicolon});
            },
            ":" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::Colon});
            },
            "." => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::Dot});
            },
            "," => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::Comma});
            },
            ")" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::RightParen});
            },
            "[" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::LeftBrace});
            },
            "]" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::RightBrace});
            },    
            "{" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::LeftCurly});
            },
            "}" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::RightCurly});
            },
            "+" | "-" | "*" | "/"=> {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::BinOp});
            },

            _ => {
                if source[0].chars().collect::<Vec<char>>()[0].is_whitespace() {
                    source.remove(0);
                    continue;
                }

                let mut ta = String::new();

                if source.len() > 0 && source[0].chars().next().unwrap().is_numeric() {
                    let mut ta = String::new();
                    let mut i = 0;
                    let mut dot_seen = false;

                    while i < source.len() {
                        let ch = source[i].chars().next().unwrap();
                        if ch.is_numeric() {
                            ta += source[i].as_str();
                            i += 1;
                        } else {
                            break;
                        }
                    }

                    if i < source.len() && source[i] == "." && i + 1 < source.len() && source[i + 1].chars().next().unwrap().is_numeric() {
                        ta += source[i].as_str(); 
                        i += 1;

                        while i < source.len() && source[i].chars().next().unwrap().is_numeric() {
                            ta += source[i].as_str();
                            i += 1;
                        }
                    }

                    if !ta.is_empty() {
                        tokens.push(Token {
                            value: ta.clone(),
                            value_type: TokenType::Number,
                        });
                        source.drain(..i);
                        continue;
                    }
                }

               if source[0] == "<"{
                   let mut ta = String::new();

                   while !ta.ends_with(">") && source.len() > 0 {
                       ta += source.remove(0).as_str();
                   }

                   tokens.push(Token{value: ta.clone(), value_type: TokenType::Flag(get_flag(parse_flag_head(ta.clone().as_str()).as_str(), get_attr(parse_attr( ta.clone().as_str() ))).unwrap())});
                   continue;
               } 

               if source.len() > 0 && !source[0].chars().collect::<Vec<char>>()[0].is_whitespace() {
                   let mut i = 0;
                   let mut ta = String::new();
                    while source.len() > i && !source[i].chars().collect::<Vec<char>>()[0].is_whitespace(){
                        ta += source[i].as_str();
                        i += 1;

                    }
                    if let Some(tax) = keywords.get(ta.as_str()){
                        tokens.push(Token{value: ta.clone(), value_type: tax.clone()});
                        source.drain(..i);
                        continue;
                    }
                }

               if source.len() > 0 && is_identifier(source[0].as_str()) {
                   let mut isk = false;
                   while source.len() > 0 && is_identifier(source[0].as_str()){
                       ta += source.remove(0).as_str();
                   }
                   tokens.push(Token{value: ta.clone(), value_type: TokenType::Identifier});
                   continue;
               }
               
               println!("{:?}", source[0]);
               panic!("Token not found");
            }
        }
    }
    tokens.push(Token{value:String::new(), value_type: TokenType::EOF});

    if PRINT_{
        println!("\n-------------------------- Lexer -------------------------------\n");
        println!("{:?}\n", tokens);
        println!("-------------------------- Abstract Syntax Tree -------------------------------\n");
    }
    return tokens;
}   

fn parse_flag_head(s: &str) -> String {
    match s.find(':') {
        Some(idx) => {
            format!("{}>", s[..idx].trim_end())
        }
        None => s.trim_end().to_string(),
    }
}

fn parse_attr(s: &str) -> Option<&str> {
    if let Some(colon_idx) = s.find(':') {
        let after_colon = &s[colon_idx + 1..];
        let end_idx = after_colon.find('>').unwrap_or(after_colon.len());
        Some(after_colon[..end_idx].trim())
    } else {
        None
    }
}

fn is_identifier(c: &str) -> bool {
    return c.chars().collect::<Vec<char>>()[0].is_alphabetic() || c.chars().collect::<Vec<char>>()[0] == '_';
}
