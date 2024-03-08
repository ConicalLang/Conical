<program> ::= <function> | <assignment>+
<function> ::= <type> <ID> "(" (<type> <ID> ", ")* ")" <block>
<expression> ::= (<binary> | <unary> " " )+
<block> ::= "{\n" <program> "}\n"
<binary> ::= <lit> <bop> <lit>
<unary> ::= <uop> <lit>
<lit> ::=  ([0-9] [1-9]+) | ("\"" [a-z]+ "\"")
<assignment> ::= <ID> " = " <expression> ";\n"
<ID> ::= ([a-z]+) [0-9]*
<bop> ::= " + " | " - " | " * " | " / "
<uop> ::= " ~" | " !"
<type> ::= "int " | "char " | "float " | "void " | "struct " | "double "