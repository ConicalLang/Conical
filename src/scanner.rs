use crate::lexer::{Token, TokenType};
#[allow(unused)]
pub struct Scanner{
    current: usize,
    elements: Vec<Token>,
}

impl Scanner{
    pub fn new(elems: Vec<Token>) -> Self{
        Self { current: 0, elements: elems }
    }
    pub fn get(&self) -> Option<&Token>{
        if self.elements.is_empty() {
            return None;
        }
        let m = if self.current == 0{0}else{self.current-1};
        Some(&self.elements[m])
    }
    pub fn next(&mut self) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        
        let out = Some(&self.elements[self.current]);
        self.current+=1;
        out
    }

    pub fn has_next(&self) -> bool{
        self.peek().is_some()
    }
    pub fn peek(&self) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        Some(&self.elements[self.current])
    }
    pub fn peek_back(&self) -> Option<&Token>{
        if self.current == 0{
            return None;
        }
        Some(&self.elements[self.current - 1])
    }
    pub fn back (&mut self) -> Option<&Token>{
        if self.current == 0 {
            return None;
        }
        let out = Some(&self.elements[self.current]);
        self.current -= 1;
        out
    }
    pub fn next_if<F>(&mut self, matcher: F) ->Option<&Token> where F: Fn(&Token) -> bool {
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
    pub fn next_if_equals(&mut self, token_type: TokenType) -> Option<&Token>{
        self.next_if(|tok|{matches!(tok.token, token_type)})
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.elements.iter()
    }
    pub fn peek_nth(&self, i: usize) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        Some(&self.elements[self.current + (i-1)])
    }

    pub fn next_while<F>(&mut self, mut matcher: F) where F: FnMut(&Token) -> bool{
        while matcher(self.next().unwrap()){}
    }
    
}