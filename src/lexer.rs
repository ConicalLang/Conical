use std::{iter::{self, from_fn}, vec};
#[derive(Debug, Clone, Copy)]
pub enum Types{
    BOOL, I8, I16, I32, I64, U8, U16, U32, U64, USIZE, F32, F64, CHAR, FUNCTION,
}
#[derive(Debug, Clone, Copy)]
pub enum Controls{
    BREAK, CONTINUE, SWITCH, RETURN, FOR, WHILE, IF, ELSE, DO, STRUCT, ENUM, TYPE, TYPEDEF, GOTO, TRUE, FALSE, IMPL,
    SIZEOF, TYPEOF, DEFAULT,
}
#[derive(Debug, Clone, Copy)]
pub enum Keywords{
    Type(Types),
    Control(Controls)
}
#[derive(Debug, Clone, Copy)]
pub enum Ops{
    EQL, EQLEQL,
    NOTEQL, GRT,
    LST, GRTEQL,
    LSTEQL,
    OR, NOT, AND, XOR,
    LOR, LAND,
    ARROW, DOT,
    DIV, PLUS, MINUS, STAR,
}
#[derive(Debug, Clone, Copy)]
pub enum Syntaxs{
    SEMICOLON,
    OPENBRACKET,CLOSEBRACKET,
    OPENPAREN, CLOSEPAREN,
    OPENSQR, CLOSESQR,
    COMMA,
}
#[derive(Debug, Clone, Copy)]
pub enum TokenType{
    LITERAL,
    IDENTIFIER,
    KEYWORD(Keywords),
    SYNTAX(Syntaxs),
    OP(Ops),
    EOF
}

#[derive(Debug, Clone)]
pub enum LiteralType{
    NUMBER(usize),
    DECIMAL(f64),
    STRING(String),
    CHARACTER(char),
    BOOL(bool),
}


#[derive(Debug, Clone)]
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

impl TokenType{
    pub fn is_operator(self) -> bool{
        match self{
            TokenType::OP(_) => true,
            _ => false,
        }
    }
    pub fn is_type(self) -> bool{
        match self{
            TokenType::KEYWORD(Keywords::Type(_)) => true,
            _ => false,
        }
    }
    pub fn is_control(self) -> bool{
        match self{
            TokenType::KEYWORD(Keywords::Control(_)) => true,
            _ => false,
        }
    }
    pub fn is_delim(self) -> bool{
        match self{
            TokenType::SYNTAX(_) => true,
            _ => false,
        }
    }
    pub fn is_keyword(self) -> bool{
        self.is_control() || self.is_type()
    }
    pub fn is_identifier(self) -> bool{
        match self{
            TokenType::IDENTIFIER => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool{
        match self{
            TokenType::LITERAL => true,
            _ => false,
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
            ' ' => {},
            ';' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::SEMICOLON), None, line)),
            '{' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::OPENBRACKET), None, line)),
            '}' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::CLOSEBRACKET), None, line)),
            '(' =>out.push(Token::new(TokenType::SYNTAX(Syntaxs::OPENPAREN), None, line)),
            ')' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::CLOSEPAREN), None, line)),
            '[' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::OPENSQR), None, line)),
            ']' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::CLOSESQR), None, line)),
            ',' => out.push(Token::new(TokenType::SYNTAX(Syntaxs::COMMA), None, line)),
            '~' => out.push(Token::new(TokenType::OP(Ops::XOR), None, line)),
            '*' => out.push(Token::new(TokenType::OP(Ops::STAR), None, line)),
            '+' => out.push(Token::new(TokenType::OP(Ops::PLUS), None, line)),
            '.' => out.push(Token::new(TokenType::OP(Ops::DOT), None, line)),
            '=' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::OP(Ops::EQLEQL), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::EQL), None, line));
                    }
                }
            },
            '!' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::OP(Ops::NOTEQL), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::NOT), None, line));
                    }
                }
            },
            '<' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::OP(Ops::LSTEQL), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::LST), None, line));
                    }
                }
            },
            '>' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '='{
                        out.push(Token::new(TokenType::OP(Ops::GRTEQL), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::GRT), None, line));
                    }
                }
            },
            '-' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '>'{
                        out.push(Token::new(TokenType::OP(Ops::ARROW), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::MINUS), None, line));
                    }
                }
            },
            '|' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '|'{
                        out.push(Token::new(TokenType::OP(Ops::LOR), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::OR), None, line));
                    }
                }
            },
            '&' => {
                if let Some((_t, n)) = src.peek(){
                    if *n == '&'{
                        out.push(Token::new(TokenType::OP(Ops::LAND), None, line));
                        src.next();
                    }else{
                        out.push(Token::new(TokenType::OP(Ops::AND), None, line));
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
                        out.push(Token::new(TokenType::OP(Ops::DIV), None, line));
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
                    out.push(Token::new(TokenType::LITERAL, Some(LiteralType::DECIMAL(ou.parse().unwrap())), line));
                }else {
                    out.push(Token::new(TokenType::LITERAL, Some(LiteralType::NUMBER(ou.parse().unwrap())), line));
                }
            },
//SWITCH, RETURN, FOR, WHILE, IF, ELSE, DO, STRUCT, ENUM, TYPE, TYPEDEF, GOTO, TRUE, FALSE, 
//BOOL, I8, I16, I32, I64, U8, U16, U32, U64, USIZE, F32, F64, CHAR, FUNCTION, SIZEOF, TYPEOF, DEFAULT,
            n if n.is_alphabetic()=>{
                let ou: String = iter::once(token)
                .chain(from_fn(|| src.by_ref().next_if(|(_t, s)| s.is_alphanumeric()).map(|(_t, r)| {r})))
                .collect::<String>();
                let mut out_option = None;
                
                let mut out_type: TokenType =
                match ou.as_str(){
                    "break" => TokenType::KEYWORD(Keywords::Control(Controls::BREAK)),
                    "continue" => TokenType::KEYWORD(Keywords::Control(Controls::CONTINUE)),
                    "switch" => TokenType::KEYWORD(Keywords::Control(Controls::SWITCH)),
                    "return" => TokenType::KEYWORD(Keywords::Control(Controls::RETURN)),
                    "for" => TokenType::KEYWORD(Keywords::Control(Controls::FOR)),
                    "while" => TokenType::KEYWORD(Keywords::Control(Controls::WHILE)),
                    "if" => TokenType::KEYWORD(Keywords::Control(Controls::IF)),
                    "else" => TokenType::KEYWORD(Keywords::Control(Controls::ELSE)),
                    "do" => TokenType::KEYWORD(Keywords::Control(Controls::DO)),
                    "struct" => TokenType::KEYWORD(Keywords::Control(Controls::STRUCT)),
                    "enum" => TokenType::KEYWORD(Keywords::Control(Controls::ENUM)),
                    "type" => TokenType::KEYWORD(Keywords::Control(Controls::TYPE)),
                    "typedef" => TokenType::KEYWORD(Keywords::Control(Controls::TYPEDEF)),
                    "goto" => TokenType::KEYWORD(Keywords::Control(Controls::GOTO)),
                    "true" => TokenType::KEYWORD(Keywords::Control(Controls::TRUE)),
                    "false" => TokenType::KEYWORD(Keywords::Control(Controls::FALSE)),
                    "sizeof" => TokenType::KEYWORD(Keywords::Control(Controls::SIZEOF)),
                    "typeof" => TokenType::KEYWORD(Keywords::Control(Controls::TYPEOF)),
                    "default" => TokenType::KEYWORD(Keywords::Control(Controls::DEFAULT)),
                    "impl" => TokenType::KEYWORD(Keywords::Control(Controls::IMPL)),
                    "bool" => TokenType::KEYWORD(Keywords::Type(Types::BOOL)),
                    "i8" => TokenType::KEYWORD(Keywords::Type(Types::I8)),
                    "i16" => TokenType::KEYWORD(Keywords::Type(Types::I16)),
                    "i32" => TokenType::KEYWORD(Keywords::Type(Types::I32)),
                    "i64" => TokenType::KEYWORD(Keywords::Type(Types::I64)),
                    "u8" => TokenType::KEYWORD(Keywords::Type(Types::U8)),
                    "u16" => TokenType::KEYWORD(Keywords::Type(Types::U16)),
                    "u32" => TokenType::KEYWORD(Keywords::Type(Types::U32)),
                    "u64" => TokenType::KEYWORD(Keywords::Type(Types::U64)),
                    "f32" => TokenType::KEYWORD(Keywords::Type(Types::F32)),
                    "f64" => TokenType::KEYWORD(Keywords::Type(Types::F64)),
                    "usize" => TokenType::KEYWORD(Keywords::Type(Types::USIZE)),
                    "char" => TokenType::KEYWORD(Keywords::Type(Types::CHAR)),
                    "func" => TokenType::KEYWORD(Keywords::Type(Types::FUNCTION)),
                    

                    m => {
                    out_option = Some(LiteralType::STRING(String::from(m)));
                    
                    TokenType::IDENTIFIER
                }
                };
                out.push(Token::new(out_type, out_option, line))
            },
            cx =>{panic!("Unknown character: {}, at line: {}", cx, line);}
        }
    }
    out.push(Token::new(TokenType::EOF,  None, line));
    out
}
