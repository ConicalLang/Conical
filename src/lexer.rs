use std::{collections::binary_heap::Iter, env, fs, iter::{self, from_fn}, vec};


#[derive(Debug, Clone, Copy)]
pub enum TokenType{
    //LITERALS
    LITERAL = 0,
    IDENTIFIER,
    SEMICOLON,
    OPENBRACKET,CLOSEBRACKET,
    OPENPAREN, CLOSEPAREN,
    OPENSQR, CLOSESQR,
    EQL, EQLEQL,
    NOTEQL, GRT,
    LST, GRTEQL,
    LSTEQL,
    OR, NOT, AND, XOR,
    LOR, LAND,
    ARROW, DOT,
    COMMA,
    DIV, PLUS, MINUS, STAR,
    //keywords
    BREAK, CONTINUE, SWITCH, RETURN, FOR, WHILE, IF, ELSE, DO, STRUCT, ENUM, TYPE, TYPEDEF, GOTO, TRUE, FALSE, IMPL,
    BOOL, I8, I16, I32, I64, U8, U16, U32, U64, USIZE, F32, F64, CHAR, FUNCTION, SIZEOF, TYPEOF, DEFAULT,
    EOF
}

#[derive(Debug)]
pub enum LiteralType{
    NUMBER(usize),
    DECIMAL(f64),
    STRING(String),
    CHARACTER(char),
    BOOL(bool),
}
#[derive(Debug)]
pub enum GeneralTokenType {
    OPERATOR,
    KEYWORD,
    DELIM,
    BRACKET
}

#[derive(Debug)]
pub struct Token{
    pub token: TokenType,
    pub value: Option<LiteralType>,
    pub token_type: Option<GeneralTokenType>,
    pub line: i32,
}
impl Token{
    fn new(token: TokenType, token_type: Option<GeneralTokenType>, value: Option<LiteralType>, line: i32) -> Self{
        Self{
            token: token,
            value: value,
            line: line,
            token_type: token_type,
        }
    }
    
    
}


// pub static OPS: Vec<TokenType> = vec![TokenType::EQL, TokenType::EQLEQL,
// TokenType::NOTEQL, TokenType::GRT,
// TokenType::LST, TokenType::GRTEQL,
// TokenType::LSTEQL,
// TokenType::OR, TokenType::NOT, TokenType::AND, TokenType::XOR,
// TokenType::LOR, TokenType::LAND,
// TokenType::ARROW, TokenType::DOT,TokenType::DIV, TokenType::PLUS, TokenType::MINUS, TokenType::STAR];

// pub static KEYWORDS: Vec<TokenType> = vec![TokenType::BREAK, TokenType::CONTINUE, TokenType::SWITCH, TokenType::RETURN, TokenType::FOR, 
// TokenType::WHILE, TokenType::IF, TokenType::ELSE, TokenType::DO, TokenType::STRUCT, TokenType::ENUM, TokenType::TYPE, TokenType::TYPEDEF, 
// TokenType::GOTO, TokenType::TRUE, TokenType::FALSE, TokenType::IMPL,TokenType::BOOL, TokenType::I8, TokenType::I16, TokenType::I32, TokenType::I64, 
// TokenType::U8, TokenType::U16, TokenType::U32, TokenType::U64, TokenType::USIZE, TokenType::F32, TokenType::F64, TokenType::CHAR, TokenType::FUNCTION, 
// TokenType::SIZEOF, TokenType::TYPEOF, TokenType::DEFAULT];

// pub static TYPES: Vec<TokenType> = vec![TokenType::BOOL, TokenType::I8, TokenType::I16, TokenType::I32, TokenType::I64, 
// TokenType::U8, TokenType::U16, TokenType::U32, TokenType::U64, TokenType::USIZE, TokenType::F32, TokenType::F64, TokenType::CHAR, TokenType::FUNCTION];

pub fn lex(input: &str) -> Vec<Token>{
    let mut out: Vec<Token> = vec![];
    let mut line = 1;
    
    
    let mut src = input.char_indices().peekable();
    while let Some((_i, token)) = src.next(){
        match token{
            '\n' | '\r' => line+=1,
            ' ' => {},
            ';' => out.push(Token::new(TokenType::SEMICOLON, Some(GeneralTokenType::DELIM), None, line)),
            '{' => out.push(Token::new(TokenType::OPENBRACKET, Some(GeneralTokenType::BRACKET),None, line)),
            '}' => out.push(Token::new(TokenType::CLOSEBRACKET, Some(GeneralTokenType::BRACKET),None, line)),
            '(' => out.push(Token::new(TokenType::OPENPAREN, Some(GeneralTokenType::BRACKET),None, line)),
            ')' => out.push(Token::new(TokenType::CLOSEPAREN, Some(GeneralTokenType::BRACKET),None, line)),
            '[' => out.push(Token::new(TokenType::OPENSQR, Some(GeneralTokenType::BRACKET),None, line)),
            ']' => out.push(Token::new(TokenType::CLOSESQR, Some(GeneralTokenType::BRACKET),None, line)),
            '~' => out.push(Token::new(TokenType::XOR, Some(GeneralTokenType::OPERATOR),None, line)),
            ',' => out.push(Token::new(TokenType::COMMA, Some(GeneralTokenType::DELIM),None, line)),
            '*' => out.push(Token::new(TokenType::STAR, Some(GeneralTokenType::OPERATOR), None, line)),
            '+' => out.push(Token::new(TokenType::PLUS, Some(GeneralTokenType::OPERATOR), None, line)),
            '.' => out.push(Token::new(TokenType::DOT, None, None, line)),
            '=' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::EQLEQL, Some(GeneralTokenType::OPERATOR),None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::EQL, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '!' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::NOTEQL, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::NOT, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '<' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::LSTEQL, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::LST, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '>' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::GRTEQL, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::GRT, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '-' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '>'{
                        out.push(Token::new(TokenType::ARROW, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::MINUS, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '|' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '|'{
                        out.push(Token::new(TokenType::LOR, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OR, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '&' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '&'{
                        out.push(Token::new(TokenType::LAND, Some(GeneralTokenType::OPERATOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::AND, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '/' => {
                if let Some((_t, nlk)) = src.peek(){
                    if *nlk == '/'{
                        src.next();
                        let _ou: String = iter::once(token)
                        .chain(from_fn(|| src.by_ref().next_if(|(_t, n)| {*n != '\n'}).map(|(_t, r)|{r})))
                        .collect();
                        
                    }else{
                        out.push(Token::new(TokenType::DIV, Some(GeneralTokenType::OPERATOR), None, line));
                    }
                }
            },
            '"' => {
                
                let mut ou = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, n)| {*n != '"' && *n != '\n' })
                .map(|(_t, r)|{r})))
                .collect::<String>();
                ou.remove(0);
                out.push(Token::new(TokenType::LITERAL, None, Some(LiteralType::STRING(ou)), line));
                src.next();

            },
            
            n if n.is_numeric() => {
                let ou: String = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, s)| s.is_ascii_digit() || *s == '.').map(|(_t, r)| {r})))
                .collect::<String>();
                if ou.contains('.') {
                    out.push(Token::new(TokenType::LITERAL, None, Some(LiteralType::DECIMAL(ou.parse().unwrap())), line))
                }else {
                    out.push(Token::new(TokenType::LITERAL, None, Some(LiteralType::NUMBER(ou.parse().unwrap())), line))
                }
            },
//SWITCH, RETURN, FOR, WHILE, IF, ELSE, DO, STRUCT, ENUM, TYPE, TYPEDEF, GOTO, TRUE, FALSE, 
//BOOL, I8, I16, I32, I64, U8, U16, U32, U64, USIZE, F32, F64, CHAR, FUNCTION, SIZEOF, TYPEOF, DEFAULT,
            n if n.is_alphabetic()=>{
                let ou: String = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, s)| s.is_alphanumeric()).map(|(_t, r)| {r})))
                .collect::<String>();
                let mut out_option = None;
                let mut out_gen = Some(GeneralTokenType::KEYWORD);
                let mut out_type: TokenType =
                match ou.as_str(){
                    "break" => TokenType::BREAK,
                    "continue" => TokenType::CONTINUE,
                    "switch" => TokenType::SWITCH,
                    "return" => TokenType::RETURN,
                    "for" => TokenType::FOR,
                    "while" => TokenType::WHILE,
                    "if" => TokenType::IF,
                    "else" => TokenType::ELSE,
                    "do" => TokenType::DO,
                    "struct" => TokenType::STRUCT,
                    "enum" => TokenType::ENUM,
                    "type" => TokenType::TYPE,
                    "typedef" => TokenType::TYPEDEF,
                    "goto" => TokenType::GOTO,
                    "true" => TokenType::TRUE,
                    "false" => TokenType::FALSE,
                    "bool" => TokenType::BOOL,
                    "i8" => TokenType::I8,
                    "i16" => TokenType::I16,
                    "i32" => TokenType::I32,
                    "i64" => TokenType::I64,
                    "u8" => TokenType::U8,
                    "u16" => TokenType::U16,
                    "u32" => TokenType::U32,
                    "u64" => TokenType::U64,
                    "f32" => TokenType::F32,
                    "f64" => TokenType::F64,
                    "usize" => TokenType::USIZE,
                    "char" => TokenType::CHAR,
                    "func" => TokenType::FUNCTION,
                    "sizeof" => TokenType::SIZEOF,
                    "typeof" => TokenType::TYPEOF,
                    "default" => TokenType::DEFAULT,
                    "impl" => TokenType::IMPL,

                    m => {
                    out_option = Some(LiteralType::STRING(String::from(m)));
                    out_gen = None;
                    TokenType::IDENTIFIER
                }
                };
                out.push(Token::new(out_type, out_gen, out_option, line))
            },
            cx =>{panic!("Unknown character: {}, at line: {}", cx, line);}
        }
    }
    out.push(Token::new(TokenType::EOF, None, None, line));
    out
}
