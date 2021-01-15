# rs9cc

mini C compiler written in Rust. This is my hobby project. I use [compilerbook](https://www.sigbus.info/compilerbook) as a reference for this project.

**Note**: This project is work in progress.

## EBNF
```
program                 = (function | declaration ("=" initialize)? ";" | func-prototype )*
type-specifier          = builtin-type | struct-dec | typedef-name | enum-specifier"
builtin-type            = "void" 
                        | "_Bool"
                        | "char" 
                        | "short" | "short" "int" | "int" "short" 
                        | "int" 
                        | "long" | "int" "long" | "long" "int" 
declarator              = "*"* ("(" declarator ")" | ident) type-suffix
abstract-declarator     = "*"* ("(" declarator ")")? type-suffix
type-suffix             = ("[" num? "]" type-suffix)?
type-name               = type-specifier abstract-declarator type-suffix
struct-dec              = "struct" ident? ("{" declaration ";" "}")?
enum-specifier          = enum ident? "{" enum-list? "}"
                        | enum ident
enum-list               = ident ("=" num)? ("," ident ("=" num)?)* ","?
declaration             = type-specifier declarator type-suffix
                        | type-specifier  
initialize              = "{" (expr ("," expr)*)? "}" 
                        | expr 
func-prototype          = type-specifier declarator "(" params? ")" 
function                = func-prototype "{" stmt* "}"
params                  = declaration ("," declaration)*
stmt                    = expr ";"
                        | "return" expr ";"
                        | "if" "(" expr ")" stmt
                        | "while" "(" expr ")" stmt
                        | "for" "(" stmt? ";" expr? ";" expr? ")" stmt
                        | "{" stmt* "}"
                        | declaration ("=" initialize)? ";"
                        | "break" ";" 
                        | "continue" ";"
                        | "goto" ident ";"
                        | ident ":" stmt
                        | "switch" "("expr")" stmt
                        | "case" num ":" stmt
                        | "default" ":" stmt
expr                    = assign ("," assign)*
assign                  = conditional (assign-op assign)?
assign-op               = "=" | "+=" | "-=" | "*=" | "/=" | "<<=" | ">>="
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
postfix                 | primary ("[" expr "]" | "." ident | "->" ident)*
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
```
$ ./bin/runner.sh test
```

on linux, you can use following command instead.
```
$ RS9CC_ON_WORKFLOW=1 ./bin/test.sh
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


## todo
- integer
- etc...

 # Reference
 - [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
