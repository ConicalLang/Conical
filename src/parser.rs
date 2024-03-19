





use std::{cell::RefCell, rc::Rc};

use slab_tree::{NodeId, Tree, TreeBuilder};

use crate::lexer::{Controls, Keywords, Ops, Syntaxs, Token, TokenType, Types};

use crate::scanner::*;



#[derive(Debug, Clone)]
pub enum NodeType{
    Expr,
    Stmt,
    Terminal(Token),
    Operator(Token),
    Head,
    Assignment,
    Decl,
    Generic,
    Function,


}
#[derive(Debug)]
pub struct AbstractSyntaxTree{
    tree: Tree<TreeNode>,
    cursor: NodeId,
}

impl AbstractSyntaxTree{
    pub fn new(root: TreeNode) -> Self{
        let tree = TreeBuilder::new().with_root(root).build();
        let rootid = tree.root_id().unwrap();
        Self { tree: tree, cursor: rootid}
    }
    pub fn set_cursor(&mut self, cursor: NodeId){
        self.cursor = cursor;
    }
    pub fn get_cursor(&self) -> NodeId{
        self.cursor
    }
    pub fn append(&mut self, node: TreeNode) -> NodeId{
        self.cursor = self.tree.get_mut(self.cursor).unwrap().append(node).node_id();
        self.cursor
    }
    pub fn get_tree(&self) -> &Tree<TreeNode>{
        &self.tree
    }
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


pub fn parse(tokens: Vec<Token>) -> AbstractSyntaxTree {
    
    
    
    let mut scanner = Scanner::new(tokens);
    
    let mut tree: AbstractSyntaxTree = AbstractSyntaxTree::new(TreeNode::new(NodeType::Head));

    expression(&mut scanner, &mut tree);
    
    tree
   
}

fn expression(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    equality(scanner, ast);
}
fn equality(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    comparison(scanner, ast);
    while scanner.has_next() && matches!(scanner.next().unwrap().token, TokenType::OP(Ops::EQLEQL) | TokenType::OP(Ops::NOTEQL)){
        ast.append(TreeNode::new(NodeType::Operator(scanner.peek_back().unwrap().clone())));
        comparison(scanner, ast);
    }
    
}
fn comparison(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    term(scanner, ast);
    
    while scanner.has_next() && matches!(scanner.next().unwrap().token, TokenType::OP(Ops::GRT) | TokenType::OP(Ops::GRTEQL) | TokenType::OP(Ops::LST) | TokenType::OP(Ops::LSTEQL)){
        ast.append(TreeNode::new(NodeType::Operator(scanner.peek_back().unwrap().clone())));
        term(scanner, ast);
    }
    
}

fn term(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    factor(scanner, ast);
    while scanner.has_next() && matches!(scanner.next().unwrap().token, TokenType::OP(Ops::MINUS) | TokenType::OP(Ops::PLUS)){
        ast.append(TreeNode::new(NodeType::Operator(scanner.peek_back().unwrap().clone())));
        factor(scanner, ast);
    }
    
}
fn factor(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    unary(scanner, ast);
    while scanner.has_next() && matches!(scanner.next().unwrap().token, TokenType::OP(Ops::DIV) | TokenType::OP(Ops::STAR)){
        ast.append(TreeNode::new(NodeType::Operator(scanner.peek_back().unwrap().clone())));
        unary(scanner, ast);
    }
    
}
fn unary(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    if scanner.has_next() && matches!(scanner.next().unwrap().token, TokenType::OP(Ops::BANG) | TokenType::OP(Ops::MINUS)){
        ast.append(TreeNode::new(NodeType::Operator(scanner.get().unwrap().clone())));
        unary(scanner, ast);
    }
    primary(scanner, ast);
}
fn primary(scanner: &mut Scanner, ast: &mut AbstractSyntaxTree){
    match scanner.get().unwrap().token{
        TokenType::KEYWORD(Keywords::Control(Controls::TRUE)) => {
            ast.append(TreeNode::new(NodeType::Terminal(Token::new(TokenType::LITERAL, Some(crate::lexer::LiteralType::BOOL(true)), scanner.get().unwrap().line))));

        },
        TokenType::KEYWORD(Keywords::Control(Controls::FALSE)) => {
            ast.append(TreeNode::new(NodeType::Terminal(Token::new(TokenType::LITERAL, Some(crate::lexer::LiteralType::BOOL(false)), scanner.get().unwrap().line))));
        },
        TokenType::LITERAL => {
            match &scanner.get().unwrap().value{
                Some(s) => {
                    ast.append(TreeNode::new(NodeType::Terminal(Token::new(TokenType::LITERAL, Some(s.clone()),  scanner.get().unwrap().line))));
                    scanner.next();
                },
                None => {}
            }
        },
        TokenType::SYNTAX(Syntaxs::OPENPAREN) =>{
            expression(scanner, ast);
        },
        _ => {}
    }
}




