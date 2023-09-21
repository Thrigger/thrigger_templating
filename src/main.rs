mod config;
mod token;

use token::{Token, TokenType};

use std::fs;

use clap::Parser;


fn main() {
    let args = config::Arg::parse();

    let contents = fs::read_to_string(args.template).unwrap();
    println!("Content of template:\n{contents:?}");

    let tokens: Vec<Token> = scan(&contents);

    println!("This is the result:\n");
    println!("-------------");
    for token in tokens {
        println!("{:?}", token.to_string());
    }
    println!("-----END-----");

}

fn scan(cont: &str) -> Vec<Token> {
    let mut i = 0;
    let mut tokens = vec![];

    while i < cont.len() {
        if let Some(tok) = get_one_char_token(cont, i) {
            tokens.push(tok);
            i += 1;
        } else if let Some((tok, new_i))  = get_keyword_token(cont, i) {
            tokens.push(tok);
            i = new_i;
        } else {
            i+=1;
        }
    }
    tokens
}

fn get_one_char_token(s: &str, i: usize) -> Option<Token> {
    match &s[i..i+1] {
        "(" => Some(Token::new(TokenType::LeftPara, None)),
        ")" => Some(Token::new(TokenType::RightPara, None)),
        _ => None,
    }
}
fn get_keyword_token(s: &str, i: usize) -> Option<(Token, usize)> {
    if s.len() >= i + 7 && &s[i..i+7] == "println" {
        return Some((Token::new(TokenType::Println, None), 7));
    } else if s.len() >= i + 5 && &s[i..i+5] == "print" {
        return Some((Token::new(TokenType::Print, None), 5));
    }
    None
}
