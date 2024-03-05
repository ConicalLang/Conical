use std::{collections::binary_heap::Iter, env, fs, iter::{self, from_fn}};


#[derive(Debug)]
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
pub struct Token{
    pub token: TokenType,
    pub value: Option<LiteralType>,
    pub line: i32,
}
impl Token{
    fn new(token: TokenType, value: Option<LiteralType>, line: i32) -> Self{
        Self{
            token: token,
            value: value,
            line: line,
        }
    }
    
}






pub fn lex(input: &str) -> Vec<Token>{
    let mut out: Vec<Token> = vec![];
    let mut line = 1;
    
    
    let mut src = input.char_indices().peekable();
    while let Some((_i, token)) = src.next(){
        match token{
            '\n' | '\r' => line+=1, 
            ';' => out.push(Token::new(TokenType::SEMICOLON, None, line)),
            '{' => out.push(Token::new(TokenType::OPENBRACKET, None, line)),
            '}' => out.push(Token::new(TokenType::CLOSEBRACKET, None, line)),
            '(' => out.push(Token::new(TokenType::OPENPAREN, None, line)),
            ')' => out.push(Token::new(TokenType::CLOSEPAREN, None, line)),
            '[' => out.push(Token::new(TokenType::OPENSQR, None, line)),
            ']' => out.push(Token::new(TokenType::CLOSESQR, None, line)),
            '~' => out.push(Token::new(TokenType::XOR, None, line)),
            ',' => out.push(Token::new(TokenType::COMMA, None, line)),
            '*' => out.push(Token::new(TokenType::STAR, None, line)),
            '+' => out.push(Token::new(TokenType::PLUS, None, line)),
            '.' => out.push(Token::new(TokenType::DOT, None, line)),
            '=' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::EQLEQL, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::EQL, None, line));
                    }
                }
            },
            '!' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::NOTEQL, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::NOT, None, line));
                    }
                }
            },
            '<' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::LSTEQL, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::LST, None, line));
                    }
                }
            },
            '>' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::GRTEQL, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::GRT, None, line));
                    }
                }
            },
            '-' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '>'{
                        out.push(Token::new(TokenType::ARROW, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::MINUS, None, line));
                    }
                }
            },
            '|' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '|'{
                        out.push(Token::new(TokenType::LOR, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OR, None, line));
                    }
                }
            },
            '&' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '&'{
                        out.push(Token::new(TokenType::LAND, None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::AND, None, line));
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
                        out.push(Token::new(TokenType::DIV, None, line));
                    }
                }
            },
            '"' => {
                
                let mut ou = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, n)| {*n != '"' && *n != '\n' })
                .map(|(_t, r)|{r})))
                .collect::<String>();
                ou.remove(0);
                out.push(Token::new(TokenType::LITERAL, Some(LiteralType::STRING(ou)), line));
                src.next();

            },
            
            n if n.is_numeric() => {
                let ou: String = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, s)| s.is_ascii_digit() || *s == '.').map(|(_t, r)| {r})))
                .collect::<String>();
                if ou.contains('.') {
                    out.push(Token::new(TokenType::LITERAL, Some(LiteralType::DECIMAL(ou.parse().unwrap())), line))
                }else {
                    out.push(Token::new(TokenType::LITERAL, Some(LiteralType::NUMBER(ou.parse().unwrap())), line))
                }
            },
//SWITCH, RETURN, FOR, WHILE, IF, ELSE, DO, STRUCT, ENUM, TYPE, TYPEDEF, GOTO, TRUE, FALSE, 
//BOOL, I8, I16, I32, I64, U8, U16, U32, U64, USIZE, F32, F64, CHAR, FUNCTION, SIZEOF, TYPEOF, DEFAULT,
            n if n.is_alphabetic()=>{
                let ou: String = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, s)| s.is_alphanumeric()).map(|(_t, r)| {r})))
                .collect::<String>();
                match ou.as_str(){
                    "break" => out.push(Token::new(TokenType::BREAK, None, line)),
                    "continue" => out.push(Token::new(TokenType::CONTINUE, None, line)),
                    "switch" => out.push(Token::new(TokenType::SWITCH, None, line)),
                    "return" => out.push(Token::new(TokenType::RETURN, None, line)),
                    "for" => out.push(Token::new(TokenType::FOR, None, line)),
                    "while" => out.push(Token::new(TokenType::WHILE, None, line)),
                    "if" => out.push(Token::new(TokenType::IF, None, line)),
                    "else" => out.push(Token::new(TokenType::ELSE, None, line)),
                    "do" => out.push(Token::new(TokenType::DO, None, line)),
                    "struct" => out.push(Token::new(TokenType::STRUCT, None, line)),
                    "enum" => out.push(Token::new(TokenType::ENUM, None, line)),
                    "type" => out.push(Token::new(TokenType::TYPE, None, line)),
                    "typedef" => out.push(Token::new(TokenType::TYPEDEF, None, line)),
                    "goto" => out.push(Token::new(TokenType::GOTO, None, line)),
                    "true" => out.push(Token::new(TokenType::TRUE, None, line)),
                    "false" => out.push(Token::new(TokenType::FALSE, None, line)),
                    "bool" => out.push(Token::new(TokenType::BOOL, None, line)),
                    "i8" => out.push(Token::new(TokenType::I8, None, line)),
                    "i16" => out.push(Token::new(TokenType::I16, None, line)),
                    "i32" => out.push(Token::new(TokenType::I32, None, line)),
                    "i64" => out.push(Token::new(TokenType::I64, None, line)),
                    "u8" => out.push(Token::new(TokenType::U8, None, line)),
                    "u16" => out.push(Token::new(TokenType::U16, None, line)),
                    "u32" => out.push(Token::new(TokenType::U32, None, line)),
                    "u64" => out.push(Token::new(TokenType::U64, None, line)),
                    "f32" => out.push(Token::new(TokenType::F32, None, line)),
                    "f64" => out.push(Token::new(TokenType::F64, None, line)),
                    "usize" => out.push(Token::new(TokenType::USIZE, None, line)),
                    "char" => out.push(Token::new(TokenType::CHAR, None, line)),
                    "func" => out.push(Token::new(TokenType::FUNCTION, None, line)),
                    "sizeof" => out.push(Token::new(TokenType::SIZEOF, None, line)),
                    "typeof" => out.push(Token::new(TokenType::TYPEOF, None, line)),
                    "default" => out.push(Token::new(TokenType::DEFAULT, None, line)),
                    "impl" => out.push(Token::new(TokenType::IMPL, None, line)),

                    m => out.push(Token::new(TokenType::IDENTIFIER, Some(LiteralType::STRING(String::from(m))), line))
                }
            },
            cx =>{panic!("Unknown character: {}, at line: {}", cx, line);}
        }
    }
    out.push(Token::new(TokenType::EOF, None, line));
    out
}
