use slab_tree::Tree;

use crate::lexer::{Token, TokenType};

enum ExpressionType{

}
struct Expression{
    expr_type: ExpressionType,
    

}

pub fn parse(tokens: Vec<Token>) -> Tree<Token>{
    let mut out: Tree<Token> = Tree::new();
    let mut iter = tokens.iter().peekable();
    let mut is_block = false;
    while let Some(tok) = iter.next(){
        
    }
    
    out
}