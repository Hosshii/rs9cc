# rs9cc

mini C compiler written in Rust. This is my hobby project. I use [compilerbook](https://www.sigbus.info/compilerbook) as a reference for this project.

**Note**: This project is work in progress.

## EBNF
```
program     = (function | declaration ";" )*
basetype    = "int" "*"*
function    = declaration "(" params? ")" "{" stmt* "}"
params      = declaration ("," declaration)*
stmt        = expr ";"
            | "return" expr ";"
            | "if" "(" expr ")" stmt
            | "while" "(" expr ")" stmt
            | "for" "(" expr? ";" expr? ";" expr? ")" stmt
            | "{" stmt* "}"
            | declaration ";"
expr        = assign
assign      = equality ("=" assign)?
equality    = relational ("==" relational | "!=" relational)*
relational  = add ("<" add | "<=" | ">" add | ">=" add)*
add         = mul ("+" mul | "-" mul)*
mul         = unary ("*" unary | "/" unary)*
unary       = ("+" | "-")? primary
            | "*" unary
            | "&" unary
            | "sizeof" unary
primary     = num | ident (func-args | "[" num "]")? | "(" expr ")"
func-args   = "(" (assign ("," assign)*)? ")"
declaration = basetype ident ("[" num "]")?
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


## todo
- integer
- pointer and String literal
- etc...

 # Reference
 - [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
