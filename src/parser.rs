use std::{ops::Add, vec};

use slab_tree::Tree;

use crate::lexer::{self, GeneralTokenType, Token, TokenType};

enum ExpressionType{
    BINARY(Token, Token, Token),
    UNARY(Token, Token),
    ASSIGNMENT(Token, Token, Token),
    LITERAL(Token),
}

struct Scanner{
    current: usize,
    elements: Vec<Token>,
}

impl Scanner{
    fn new(elems: Vec<Token>) -> Self{
        Self { current: 0, elements: elems }
    }
    fn get(&self) -> Option<&Token>{
        if self.elements.is_empty() {
            return None;
        }
        Some(&self.elements[self.current-1])
    }
    fn next(&mut self) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        
        let out = Some(&self.elements[self.current]);
        self.current+=1;
        out
    }
    fn peek(&self) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        Some(&self.elements[self.current])
    }
    fn back (&mut self) -> Option<&Token>{
        if self.current == 0 {
            return None;
        }
        let out = Some(&self.elements[self.current]);
        self.current -= 1;
        out
    }
    fn next_if<F>(&mut self, matcher: F) ->Option<&Token> where F: Fn(&Token) -> bool {
        match self.peek(){
            Some(tok) => {
                if matcher(tok) {
                    return self.next();
                }else{
                    None
                }
            },
            None => None 
        }
    }
    fn next_if_equals(&mut self, token_type: TokenType) -> Option<&Token>{
        self.next_if(|tok|{matches!(tok.token, token_type)})
    }

    fn iter(&self) -> impl Iterator<Item = &Token> {
        self.elements.iter()
    }
    
}





pub fn parse(tokens: Vec<Token>) -> Tree<Token>{
    let mut out: Tree<Token> = Tree::new();
    let mut scanner = Scanner::new(tokens);
    deceleration(&mut scanner);
    assignment(&mut scanner);
    function(&mut scanner);
    
    out
}
fn deceleration(scanner: &mut Scanner) {
    
}

fn expression(scanner: &mut Scanner){

}
fn assignment(scanner: &mut Scanner) {
    let id = scanner.next_if_equals(TokenType::IDENTIFIER);
    scanner.next_if_equals(TokenType::EQL);
    let expr = expression(scanner);
    if !matches!(scanner.next_if_equals(TokenType::SEMICOLON), None){
        //panic!
    }
}

fn function(scanner: &mut Scanner){
    //let rettype = scanner.next_if(|tok| {lexer::TYPES.iter().any(|t| {matches!(t.token, tok.token)}) | matches!(tok.token, TokenType::IDENTIFIER)});
    let id = scanner.next_if_equals(TokenType::IDENTIFIER);
    scanner.next_if_equals(TokenType::OPENPAREN);
    let mut params: Vec<&Token> = vec![];
    while let Some(curr) = scanner.next_if(|tok|{!matches!(tok.token, TokenType::CLOSEPAREN)})  {
        
    }
}

