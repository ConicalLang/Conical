<expression> ::= <equality>
<equality> ::= <comparison> ( ( "!=" | "==" ) <comparison> )* 
<comparison> ::= <term> ( ( ">" | ">=" | "<" | "<=" ) <term> )*
<term> ::= <factor> ( ( "-" | "+" ) <factor> )*
<factor> ::= <unary> ( ( "/" | "*" ) <unary> )*
<unary> ::= ( "!" | "-" ) <unary> | <primary>
<primary> ::= <num> | STR | "true" | "false" | "(" <expression> ")"
<num> ::= [0-9]+
