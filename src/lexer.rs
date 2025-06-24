use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Token{
    pub value: String,
    pub value_type: TokenType
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType{
    Number,
    Identifier,

    Let_k,
    Nil_k,
    Bool_true_t,
    Bool_false_t,

    Assign_f,
    Const_f,

    LeftParen, RightParen,
    BinOp,
    Semicolon,
    EOF,
}

static keywords: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("$", TokenType::Let_k);
    map.insert("!?", TokenType::Nil_k);
    map.insert("true", TokenType::Bool_true_t);
    map.insert("false", TokenType::Bool_false_t);
    map
});

pub static flags: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("<asg>", TokenType::Assign_f);
    map.insert("<const>", TokenType::Const_f);
    map
});

pub fn tokenize(src: String) -> Vec<Token>{
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
            ")" => {
                tokens.push(Token{value: source.remove(0), value_type: TokenType::RightParen});
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

               if source.len() > 0 && source[0].chars().collect::<Vec<char>>()[0].is_numeric(){
                    while source.len() > 0 && source[0].chars().collect::<Vec<char>>()[0].is_numeric(){
                        ta += source.remove(0).as_str();
                    }
                    tokens.push(Token{value: ta, value_type: TokenType::Number});
                    continue;
               }
               if source[0] == "<"{
                   let mut ta = String::new();

                   while !ta.ends_with(">") && source.len() > 0 {
                       ta += source.remove(0).as_str();
                   }

                   tokens.push(Token{value: ta.clone(), value_type: flags.get(ta.as_str()).cloned().expect("Flag Token not found")});
                   continue;
               } 

               if source.len() > 0 && !source[0].chars().collect::<Vec<char>>()[0].is_whitespace() {
                   let mut isk = false;
                   while source.len() > 0 && !source[0].chars().collect::<Vec<char>>()[0].is_whitespace(){
                       ta += source.remove(0).as_str();

                       if let Some(tax) = keywords.get(ta.as_str()){
                           tokens.push(Token{value: ta.clone(), value_type: tax.clone()});
                           isk = true;
                           break;
                       }
                   }
                   if !isk{
                       tokens.push(Token{value: ta.clone(), value_type: TokenType::Identifier});
                   }
                   continue;
               }
               
               println!("{:?}", tokens);
               panic!("Token not found");
            }
        }
    }
    
    tokens.push(Token{value:String::new(), value_type: TokenType::EOF});
    return tokens;
}   

use std::fs;

fn main() {
    let source = fs::read_to_string("code.io")
        .expect("Failed to read file 'code.io'");

    let tokens = tokenize(source);

    println!("{:?}", tokens);
}
