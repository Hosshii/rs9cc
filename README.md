# rs9cc

mini C compiler written in Rust. 

**Note**: This project is work in progress.

## EBNF
```
program     = stmt*
stmt        = expr ";"
            | "return" expr ";"
            | "if" "(" expr ")" stmt
            | "while" "(" expr ")" stmt
            | "for" "(" expr? ";" expr? ";" expr? ")" stmt
            | "{" stmt* "}"
expr        = assign
assign      = equality ("=" assign)?
equality    = relational ("==" relational | "!=" relational)*
relational  = add ("<" add | "<=" | ">" add | ">=" add)*
add         = mul ("+" mul | "-" mul)*
mul         = unary ("*" unary | "/" unary)*
unary       = ("+" | "-")? primary
primary     = num | ident | "(" expr ")"
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


## todo
- block statement
- function call
- function definition
- integer
- pointer and String literal
- etc...

 # Reference
 - [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
