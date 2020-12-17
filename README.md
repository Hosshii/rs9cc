# rs9cc

mini C compiler written in Rust. This is my hobby project. I use [compilerbook](https://www.sigbus.info/compilerbook) as a reference for this project.

**Note**: This project is work in progress.

## EBNF
```
program         = (function | declaration ";" | func-prototype )*
typekind        = "int" | "char"
basetype        = typekind "*"*
declaration     = basetype ident ("[" num "]")?
func-prototype  = declaration "(" params? ")" 
function        = func-prototype "{" stmt* "}"
params          = declaration ("," declaration)*
stmt            = expr ";"
                | "return" expr ";"
                | "if" "(" expr ")" stmt
                | "while" "(" expr ")" stmt
                | "for" "(" expr? ";" expr? ";" expr? ")" stmt
                | "{" stmt* "}"
                | declaration ";"
expr            = assign
assign          = equality ("=" assign)?
equality        = relational ("==" relational | "!=" relational)*
relational      = add ("<" add | "<=" | ">" add | ">=" add)*
add             = mul ("+" mul | "-" mul)*
mul             = unary ("*" unary | "/" unary)*
unary           = ("+" | "-")? postfix
                | "*" unary
                | "&" unary
                | "sizeof" unary
postfix         | primary ("[" expr "]")?
primary         = num 
                | ident (func-args | "[" num "]")? 
                | "(" expr ")"
                | str
func-args       = "(" (assign ("," assign)*)? ")"
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
