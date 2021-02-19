# rs9cc

mini C compiler written in Rust. This is my hobby project. I use [compilerbook](https://www.sigbus.info/compilerbook) as a reference for this project.

now this compiler can compile [chibicc(historical/old branch)](https://github.com/rui314/chibicc/tree/historical/old) by using [self.sh](https://github.com/rui314/chibicc/blob/historical/old/self.sh)(you have to add some func definition in the self.sh to compile)


## build 
```
$ cargo build
```

## test
### unit test
```
$ cargo test
```

### integrated test
docker is required

c script test
```
$ ./runner.sh test
```
shell script test
```
$ ./runner.sh test_sh
```



## implemented
- Four arithmetic operations (`+`, `-`, `*`, `/`)
- unray(`+`, `-`)
- comparison(`>`, `>=`, `<`, `<=`, `==`, `!=`)
- local variable
- return statement
- control statement(`if`, `else`, `for`, `while`)
- block statement(`{...}`)
- function call
- function definition
- array
- global variable
- pointer and String literal
- comment
- struct, enum
- bit op (!,~,|,&,^)
- goto,switch,continue,break
- extern
- lvar,gvar initializers
- va_start
- static local variable
- static function, global variable
etc


 # Reference
 - [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)

## EBNF
```
program                 = (function | declaration ("=" gvar-initializer)? ";" | func-prototype )*
type-specifier          = builtin-type | struct-dec | typedef-name | enum-specifier"
builtin-type            = "void" 
                        | "_Bool"
                        | "char" 
                        | "short" | "short" "int" | "int" "short" 
                        | "int" 
                        | "long" | "int" "long" | "long" "int" 
declarator              = "*"* ("(" declarator ")" | ident) type-suffix
abstract-declarator     = "*"* ("(" declarator ")")? type-suffix
type-suffix             = ("[" const-expr? "]" type-suffix)?
type-name               = type-specifier abstract-declarator type-suffix
struct-dec              = "struct" ident? ("{" declaration ";" "}")?
enum-specifier          = enum ident? "{" enum-list? "}"
                        | enum ident
enum-list               = enum-elem ("," enum-elem)* ","?
enum-elem               = ident ("=" const-expr)?
declaration             = type-specifier declarator type-suffix
                        | type-specifier  
initialize              = "{" (expr ("," expr)*)? "}" 
                        | expr 
func-prototype          = type-specifier declarator "(" params? ")" 
function                = func-prototype "{" stmt* "}"
params                  = declaration ("," declaration)* ("," "...")? | "void" 
stmt                    = expr ";"
                        | "return" expr? ";"
                        | "if" "(" expr ")" stmt
                        | "while" "(" expr ")" stmt
                        | "do" stmt "while" "(" expr ")" ";"
                        | "for" "(" stmt? ";" expr? ";" expr? ")" stmt
                        | "{" stmt* "}"
                        | declaration ("=" lvar-initializer)? ";"
                        | "break" ";" 
                        | "continue" ";"
                        | "goto" ident ";"
                        | ident ":" stmt
                        | "switch" "("expr")" stmt
                        | "case" const-expr ":" stmt
                        | "default" ":" stmt
lvar-initializer        = assign
                        | "{" lvar-initializer ("," lvar-initializer)* ","? "}"
expr                    = assign ("," assign)*
assign                  = conditional (assign-op assign)?
assign-op               = "=" | "+=" | "-=" | "*=" | "/=" | "<<=" | ">>=" | "&=" | "|=" | "^="
conditional             = logor ("?" expr ":" conditional)?
logor                   = logand ("||" logand)*
logand                  = bitor ("&&" bitor)*
bitor                   = bitxor ("|" bitxor)*
bitxor                  = bitand ("^" bitand)*
bitand                  = equality ("&" equality)*
equality                = relational ("==" relational | "!=" relational)*
relational              = shift ("<" shift | "<=" | ">" shift | ">=" shift)*
shift                   = add ("<<" add | ">>" add)*
add                     = mul ("+" mul | "-" mul)*
mul                     = cast ("*" cast | "/" cast)*
cast                    = "(" type-name ")" cast | unary
unary                   = ("+" | "-" | "*" | "&" | "!" | "~")? cast
                        | ("++" | "--") unary
                        | postfix
postfix                 = compound-literal
                        | primary ("[" expr "]" | "." ident | "->" ident | "++" | "--")*
compound-literal        = "(" type-name ")" "{" (gvar-initializer | lvar-initializer) "}"
stmt-expr               = "(" "{" stmt stmt* "}" ")"
primary                 = num 
                        | ident (func-args)? 
                        | "(" expr ")"
                        | str
                        | char
                        | "(" "{" stmt-expr-tail
                        | "sizeof" unary
                        | "sizeof "(" type-name ")"
func-args               = "(" (assign ("," assign)*)? ")"
```