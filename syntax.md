program -> (function | expression | statement | operator)*
function -> type IDENTIFIER((type IDENTIFIER,)*)block
type -> "int" | "char" | "float" | "void" | "struct" | TYPE | FUNCTION_ID | "double" | type"*"
expression ->  (binary | unary)+";"
block -> "{"program"}"
binary -> literal operator literal
unary -> operator literal
literal -> NUMBER | STRING
assignment -> IDENTIFIER "=" expression";"
decl -> type IDENTIFIER";"