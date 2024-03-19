use std::{env, fs};
mod lexer;
mod parser;
mod tree;
mod scanner;
#[warn(unused)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file_contents = fs::read_to_string(path).expect("Could not open file!");
    dbg!(lexer::lex(&file_contents));
    let x = parser::parse(lexer::lex(&file_contents));
    let mut out = String::new();
    x.write_formatted(&mut out).unwrap();
    println!("{}", out);
}