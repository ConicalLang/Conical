





use std::{cell::RefCell, rc::Rc};

use slab_tree::{NodeId, Tree, TreeBuilder};

use crate::lexer::{LiteralType, Ops, Syntaxs, Token, TokenType};

use crate::scanner::*;

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
    equality(scanner, ast, cursor);
}

fn equality(scanner: &mut Scanner, ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId){
    comparison(scanner, ast.clone(), cursor);
    
}
fn comparison(scanner: &mut Scanner, ast: Rc<RefCell<Tree<TreeNode>>>, cursor: NodeId){

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

