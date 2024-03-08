use std::{env, fs};
mod lexer;
mod parser;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file_contents = fs::read_to_string(path).expect("Could not open file!");
    dbg!(lexer::lex(&file_contents));
    


    
}