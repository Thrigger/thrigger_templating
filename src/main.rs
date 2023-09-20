use std::fs;

use clap::Parser;

mod config;

fn main() {
    let args = config::Arg::parse();

    let contents = fs::read_to_string(args.template).unwrap();
    println!("Content of template:\n{contents:?}");

    let parsed = parse(&contents);

    println!("This is the result:\n");
    println!("-------------");
    for (cmd, arg) in parsed {
        match cmd {
            "print" => print!("{arg}"),
            "println" => println!("{arg}"),
            _ => (),
        };
    }
    println!("-----END-----");

}

fn parse(cont: &str) -> Vec<(&str, &str)> {
    let mut i = 0;
    let mut ret = vec![];
    loop {
        // TODO shall look for first special character
        if let Some(stop_i) = cont[i..].find("(") {
            let cmd = &cont[i..stop_i];
            i = stop_i + 1;
            if let Some(stop_i) = cont[i..].find(")") {
                let args = &cont[i..stop_i];
                ret.push((cmd, args));
                i = stop_i + 1;
            } else {
                panic!("Error parsing could not find ending paranteses");
            }
        } else {
            break
        }
    }
    ret
}
