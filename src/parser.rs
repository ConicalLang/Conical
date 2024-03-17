





use std::{cell::RefCell, rc::Rc};

use slab_tree::{NodeId, Tree, TreeBuilder};

use crate::lexer::{LiteralType, Ops, Syntaxs, Token, TokenType};

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
        let m = if self.current == 0{0}else{self.current-1};
        Some(&self.elements[m])
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
    fn peek_nth(&self, i: usize) -> Option<&Token>{
        if self.current + 1 > self.elements.len(){
            return None;
        }
        Some(&self.elements[self.current + (i-1)])
    }
    
}

struct FnDecl{
    ret_type: bool,

}
#[derive(Debug, Clone)]
pub enum NodeType{
    Expr,
    Stmt,
    Terminal(Token),
    Operator,
    Head,
    Assignment,
    Decl,
    Generic,
    Function,


}
#[derive(Debug, Clone)]
pub struct TreeNode{
    is_term: bool,
    ntype: NodeType,
}

impl TreeNode{
    pub fn new(ntype: NodeType) -> Self{
        Self { is_term: matches!(ntype, NodeType::Terminal(_)), ntype: ntype }
    }
}


pub fn parse(tokens: Vec<Token>) -> Tree<TreeNode> {
    
    
    
    let mut scanner = Scanner::new(tokens);
    let tree = TreeBuilder::new().with_root(TreeNode::new(NodeType::Head)).build();
    let root = tree.root_id().unwrap();
    let treeref = Rc::new(RefCell::new(tree));
    deceleration(&mut scanner, treeref.clone(), root);
    function(&mut scanner, treeref.clone(), root);
    
    treeref.take()
}
fn deceleration(scanner: &mut Scanner, ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId ) {
    
    
   
    let curr = {
    let mut astt = ast.borrow_mut();
    let mut node = astt.get_mut(cursor).unwrap();
    let mut curr = node.append(TreeNode::new(NodeType::Decl));
   
    curr.append(TreeNode::new(NodeType::Terminal(scanner.next().unwrap().clone())));
    curr.append(TreeNode::new(NodeType::Terminal(scanner.next().unwrap().clone())));
    curr.node_id()
    };
 
    if matches!(scanner.peek().unwrap().token, TokenType::OP(Ops::EQL)){
        assignment(scanner, Rc::from(ast), curr);
    } 

}

fn expression(scanner: &mut Scanner, ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId ){
    
    let mut astt = ast.borrow_mut();
    let mut node = astt.get_mut(cursor).unwrap();
    let mut curr = node.append(TreeNode::new(NodeType::Expr));
    if scanner.get().unwrap().token.is_literal() && matches!(scanner.next().unwrap().token, TokenType::SYNTAX(Syntaxs::SEMICOLON)){
        
        
    }
    curr.append(TreeNode::new(NodeType::Terminal(scanner.next().unwrap().clone())));
    curr.append(TreeNode::new(NodeType::Terminal(scanner.next().unwrap().clone())));
    
}
fn assignment(scanner: &mut Scanner,  ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId ) {
    let id = scanner.get().unwrap().token;
    if !matches!(scanner.next().unwrap().token, TokenType::OP(Ops::EQL)) || !id.is_identifier(){
        return;
    }
    let curr = {
        let mut astt = ast.borrow_mut();
        let mut node = astt.get_mut(cursor).unwrap();
        let mut curr = node.append(TreeNode::new(NodeType::Assignment));
        curr.node_id()
    };
    expression(scanner, ast.clone(), curr);
}

fn function(scanner: &mut Scanner,  ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId ){
   
   
}

