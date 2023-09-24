mod config;
mod token;
mod parser;

use token::Token;

use std::fs;

use clap::Parser;


fn main() {
    let args = config::Arg::parse();

    let contents = fs::read_to_string(args.template).unwrap();
    println!("Content of template:\n{contents:?}");

    let tokens: Vec<Token> = parser::scan(&contents);

    println!("This is the result:\n");
    println!("-------------");
    for token in tokens {
        println!("{:?}", token.to_string());
    }
    println!("-----END-----");

}

