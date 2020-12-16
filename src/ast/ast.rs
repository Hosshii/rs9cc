use super::error::Error;
use super::util::*;
use super::NodeKind;
use super::{Context, Declaration, FuncDef, FuncDefMp, Function, GvarMp, Node, Program};
use crate::base_types::{BaseType, TypeKind};
use crate::token::{Block, KeyWord, Operator, TokenIter, TokenKind};
use std::rc::Rc;

// program     = (function | declaration ";" )*
pub fn program(iter: &mut TokenIter) -> Result<Program, Error> {
    let mut program = Program::new();
    while iter.peek() != None {
        // to distinguish global variable and function
        // read base type and ident
        // then peek next token.
        // if next token is ; or [], it is global variable
        // if next token is ( , it is function
        let mut b_type = expect_base_type(iter)?;
        let ident = expect_ident(iter)?;
        if let Some(next) = iter.next() {
            match next.kind {
                TokenKind::Reserved(Operator::LParen) => {
                    let mut fn_params = Vec::new();
                    if !consume(iter, Operator::RParen) {
                        fn_params = params(iter)?;
                        expect(iter, Operator::RParen)?;
                    }

                    let func_def = FuncDef::new(b_type.kind, ident, fn_params);
                    let checked_func_def =
                        Rc::new(check_func_def(iter, &program.func_def, func_def)?);
                    program.func_def.insert(
                        checked_func_def.ident.name.clone(),
                        checked_func_def.clone(),
                    );
                    let func = function(iter, checked_func_def, &program.g_var, &program.func_def)?;
                    program.functions.push(func);
                }
                TokenKind::SemiColon => {
                    program.g_var.insert(
                        ident.name.clone(),
                        Rc::new(check_g_var(iter, &program.g_var, b_type, ident)?),
                    );
                }
                TokenKind::Reserved(Operator::LArr) => {
                    b_type.kind.to_arr(expect_num(iter)?);
                    expect(iter, Operator::RArr)?;
                    expect_semi(iter)?;
                    program.g_var.insert(
                        ident.name.clone(),
                        Rc::new(check_g_var(iter, &program.g_var, b_type, ident)?),
                    );
                }
                _ => {}
            }
        }
    }
    Ok(program)
}

// basetype    = "int" "*"*
pub fn base_type(iter: &mut TokenIter) -> Result<Node, Error> {
    let type_kind = expect_type_kind(iter)?;
    let mut btype = BaseType::new(type_kind);
    loop {
        if consume(iter, Operator::Mul) {
            btype = BaseType::new(TypeKind::Ptr(Rc::new(btype)));
        } else {
            break;
        }
    }
    Ok(Node::new_leaf(NodeKind::BaseType(btype)))
}

// function    = basetype ident "(" params? ")" "{" stmt* "}"
pub fn function(
    iter: &mut TokenIter,
    func_def: Rc<FuncDef>,
    g_var: &GvarMp,
    func_def_mp: &FuncDefMp,
) -> Result<Function, Error> {
    // rad only block
    // define is come from func_def

    expect_block(iter, Block::LParen)?;

    let mut ctx = Context::new(g_var, func_def_mp);
    for fn_param in func_def.params.clone() {
        ctx.push_front(
            fn_param,
            ctx.lvar.as_ref().map(|lvar| lvar.offset).unwrap_or(0),
        )
    }

    let mut stmt_vec = Vec::new();
    loop {
        if consume_block(iter, Block::RParen) {
            return Ok(Function::new(func_def, ctx.lvar, ctx.lvar_count, stmt_vec));
        }
        stmt_vec.push(stmt(iter, &mut ctx)?);
    }
}

// params      = declaration ("," declaration)*
pub fn params(iter: &mut TokenIter) -> Result<Vec<Declaration>, Error> {
    let mut params = vec![declaration(iter)?];
    while consume_comma(iter) {
        params.push(declaration(iter)?);
    }
    Ok(params)
}

// stmt        = expr ";"
//             | "return" expr ";"
//             | "if" "(" expr ")" stmt
//             | "while" "(" expr ")" stmt
//             | "for" "(" expr? ";" expr? ";" expr? ")" stmt
//             | "{" stm| declaration ";"t* "}"
//             | declaration ";"
pub fn stmt(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if let Some(x) = iter.peek() {
        match x.kind {
            TokenKind::KeyWord(key) => match key {
                KeyWord::Return => {
                    iter.next();
                    let node = Node::new_unary(NodeKind::Return, expr(iter, ctx)?);
                    expect_semi(iter)?;
                    return Ok(node);
                }
                KeyWord::If => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_cond(NodeKind::If, expr(iter, ctx)?);
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    if let Some(x) = iter.peek() {
                        if x.kind == TokenKind::KeyWord(KeyWord::Else) {
                            iter.next();
                            node.els = Some(Box::new(stmt(iter, ctx)?));
                        }
                    }
                    return Ok(node);
                }
                KeyWord::While => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_cond(NodeKind::While, expr(iter, ctx)?);
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    return Ok(node);
                }
                KeyWord::For => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_none(NodeKind::For);
                    if !consume_semi(iter) {
                        node.init = Some(Box::new(expr(iter, ctx)?));
                        expect_semi(iter)?;
                    }
                    if !consume_semi(iter) {
                        node.cond = Some(Box::new(expr(iter, ctx)?));
                        expect_semi(iter)?;
                    }
                    if !consume_semi(iter) {
                        node.inc = Some(Box::new(expr(iter, ctx)?));
                    }
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    return Ok(node);
                }
                _ => (),
            },
            TokenKind::Block(block) => match block {
                Block::LParen => {
                    iter.next();
                    let mut stmt_vec = Vec::new();
                    loop {
                        if consume_block(iter, Block::RParen) {
                            return Ok(Node::new_none(NodeKind::Block(stmt_vec)));
                        }
                        stmt_vec.push(stmt(iter, ctx)?);
                    }
                }
                _ => {
                    return Err(Error::unexpected_token(
                        iter.s,
                        x.clone(),
                        TokenKind::Block(Block::LParen),
                    ))
                }
            },
            _ => (),
        }
    }

    if let Some(dec) = consume_declaration(iter) {
        if let Some(_) = ctx.find_lvar(&dec.ident.name) {
            // consume_declaration calls iter.next();
            // so if the variable is not defined, the error position is not correct.
            // ex
            // a = 3;
            //   ^ variable a is not defined
            // to prevent this, subtract from iter.pos.bytes.
            // but now i dont have good solution.
            return Err(Error::re_declare(iter.s, dec.ident, iter.pos, None));
        } else {
            ctx.push_front(
                dec.clone(),
                ctx.lvar.as_ref().map(|lvar| lvar.offset).unwrap_or(0),
            );
            expect_semi(iter)?;
            return Ok(Node::new_leaf(NodeKind::Declaration(dec)));
        }
    }
    let node = expr(iter, ctx)?;
    expect_semi(iter)?;
    Ok(node)
}

// expr        = assign
pub fn expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    assign(iter, ctx)
}

// assign      = equality ("=" assign)?
pub fn assign(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = equality(iter, ctx)?;
    if consume(iter, Operator::Assign) {
        let rhs = assign(iter, ctx)?;
        let lhs_type = node
            .get_type()
            .unwrap_or(TypeKind::_Invalid("invalid lhs".to_string()));
        let rhs_type = rhs
            .get_type()
            .unwrap_or(TypeKind::_Invalid("invalid rhs".to_string()));

        // 配列とポインタの比較が中途半端
        if lhs_type != rhs_type && !TypeKind::partial_comp(&lhs_type, &rhs_type) {
            return Err(Error::invalid_assignment(
                iter.s, iter.pos, lhs_type, rhs_type,
            ));
        }
        node = Node::new(NodeKind::Assign, node, rhs);
    }
    return Ok(node);
}

// equality    = relational ("==" relational | "!=" relational)*
pub fn equality(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = relational(iter, ctx)?;
    loop {
        if consume(iter, Operator::Equal) {
            node = Node::new(NodeKind::Equal, node, relational(iter, ctx)?);
        } else if consume(iter, Operator::Neq) {
            node = Node::new(NodeKind::Neq, node, relational(iter, ctx)?);
        } else {
            return Ok(node);
        }
    }
}

// relational  = add ("<" add | "<=" | ">" add | ">=" add)*
pub fn relational(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = add(iter, ctx)?;
    loop {
        if consume(iter, Operator::Lesser) {
            node = Node::new(NodeKind::Lesser, node, add(iter, ctx)?);
        } else if consume(iter, Operator::Leq) {
            node = Node::new(NodeKind::Leq, node, add(iter, ctx)?);
        } else if consume(iter, Operator::Greater) {
            // 左右を入れ替えて読み変える
            node = Node::new(NodeKind::Lesser, add(iter, ctx)?, node);
        } else if consume(iter, Operator::Geq) {
            // 左右を入れ替えて読み変える
            node = Node::new(NodeKind::Leq, add(iter, ctx)?, node);
        } else {
            return Ok(node);
        }
    }
}

// add         = mul ("+" mul | "-" mul)*
pub fn add(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = mul(iter, ctx)?;
    loop {
        if consume(iter, Operator::Plus) {
            node = Node::new(NodeKind::Add, node, mul(iter, ctx)?)
        } else if consume(iter, Operator::Minus) {
            node = Node::new(NodeKind::Sub, node, mul(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// mul         = unary ("*" unary | "/" unary)*
pub fn mul(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = unary(iter, ctx)?;
    loop {
        if consume(iter, Operator::Mul) {
            node = Node::new(NodeKind::Mul, node, unary(iter, ctx)?)
        } else if consume(iter, Operator::Div) {
            node = Node::new(NodeKind::Div, node, unary(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// unary       = ("+" | "-")? primary
//             | "*" unary
//             | "&" unary
//             | "sizeof" unary
pub fn unary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::Plus) {
        return primary(iter, ctx);
    } else if consume(iter, Operator::Minus) {
        return Ok(Node::new(
            NodeKind::Sub,
            Node::new_num(0),
            primary(iter, ctx)?,
        ));
    } else if consume(iter, Operator::Mul) {
        return Ok(Node::new_unary(NodeKind::Deref, unary(iter, ctx)?));
    } else if consume(iter, Operator::Ampersand) {
        return Ok(Node::new_unary(NodeKind::Addr, unary(iter, ctx)?));
    } else if consume(iter, Operator::Sizeof) {
        let node = unary(iter, ctx)?;
        match node.kind {
            NodeKind::Num(_) => return Ok(Node::new_num(4)),
            NodeKind::Lvar(x) => return Ok(Node::new_num(x.dec.base_type.kind.size() as u64)),
            NodeKind::Gvar(x) => return Ok(Node::new_num(x.size)),
            NodeKind::Func(func_def, _) => return Ok(Node::new_num(func_def.type_kind.size())),
            NodeKind::Deref => {
                let (deref_count, lvar) = count_deref(&node);
                let lvar = match lvar {
                    Ok(x) => x,
                    Err(x) => {
                        return Err(Error::invalid_value_dereference(
                            iter.s,
                            iter.pos,
                            x.as_str(),
                        ))
                    }
                };

                return if let Ok(type_kind) = node.get_type() {
                    Ok(Node::new_num(type_kind.size()))
                } else {
                    Err(Error::invalid_variable_dereference(
                        iter.s,
                        iter.pos,
                        (*lvar).clone(),
                        deref_count,
                    ))
                };
            }
            _ => match node.lhs.unwrap().kind {
                NodeKind::Num(_) => {
                    return Ok(Node::new_num(4));
                }
                NodeKind::Lvar(x) => return Ok(Node::new_num(x.dec.base_type.kind.size() as u64)),
                _ => unreachable!(),
            },
        }
    }
    return primary(iter, ctx);
}

// primary     = num | ident (func-args | "[" num "]")? | "(" expr ")"
pub fn primary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::LParen) {
        let node = expr(iter, ctx)?;
        expect(iter, Operator::RParen)?;
        return Ok(node);
    }

    if let Some(ident) = consume_ident(iter) {
        // todo
        // 関数もローカル変数と同じように定義済みかどうか判定するようにした方がいい
        if consume(iter, Operator::LParen) {
            let func_def = ctx
                .func_def_mp
                .get(&ident.name)
                .ok_or(Error::undefined_function(iter.s, ident, iter.pos, None))?;
            return Ok(Node::new_leaf(NodeKind::Func(
                func_def.clone(),
                func_args(iter, ctx)?,
            )));
        }
        if let Some(lvar) = ctx.find_lvar(&ident.name) {
            if consume(iter, Operator::LArr) {
                let cur = iter.pos;
                let _ = expr(iter, ctx)?;
                let end = iter.pos;
                // let num = expect_num(iter)?;
                expect(iter, Operator::RArr)?;
                let ipt = format!("*({} + ({}))", &ident.name, &iter.s[cur.bytes..end.bytes]);
                let mut tk = crate::token::tokenize(&ipt);
                return Ok(unary(&mut tk, ctx)?);
            }
            return Ok(Node::new_leaf(NodeKind::Lvar(lvar)));
        } else if let Some(x) = ctx.gvar.get(&ident.name) {
            if consume(iter, Operator::LArr) {
                let cur = iter.pos;
                let _ = expr(iter, ctx)?;
                let end = iter.pos;
                // let num = expect_num(iter)?;
                expect(iter, Operator::RArr)?;
                let ipt = format!("*({} + ({}))", &ident.name, &iter.s[cur.bytes..end.bytes]);
                let mut tk = crate::token::tokenize(&ipt);
                return Ok(unary(&mut tk, ctx)?);
            }
            return Ok(Node::new_leaf(NodeKind::Gvar(x.clone())));
        } else {
            return Err(Error::undefined_variable(iter.s, ident, iter.pos, None));
        }
    }
    return Ok(Node::new_num(expect_num(iter)?));
}

// func-args   = "(" (assign ("," assign)*)? ")"
fn func_args(iter: &mut TokenIter, ctx: &mut Context) -> Result<Vec<Node>, Error> {
    if consume(iter, Operator::RParen) {
        return Ok(vec![]);
    }
    let mut args = vec![assign(iter, ctx)?];
    while consume_comma(iter) {
        args.push(assign(iter, ctx)?);
    }
    expect(iter, Operator::RParen)?;
    Ok(args)
}

//declaration = basetype ident ("[" num "]")?
pub(crate) fn declaration(iter: &mut TokenIter) -> Result<Declaration, Error> {
    let mut b_type = expect_base_type(iter)?;
    let ident = expect_ident(iter)?;
    if consume(iter, Operator::LArr) {
        b_type.kind.to_arr(expect_num(iter)?);
        expect(iter, Operator::RArr)?;
    }
    Ok(Declaration::new(b_type, ident))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Ident, Lvar, NodeKind};
    use crate::base_types::TypeKind;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn test_expr() {
        use crate::token;
        let tests = [
            ("1==10", make_test_node(NodeKind::Equal, 1, 10)),
            ("1 != 10", make_test_node(NodeKind::Neq, 1, 10)),
            ("1  <10", make_test_node(NodeKind::Lesser, 1, 10)),
            ("1<=10", make_test_node(NodeKind::Leq, 1, 10)),
            ("1>10", make_test_node(NodeKind::Lesser, 10, 1)), // Lesser,LeqはGreater.Geqを使って実装されてる
            ("1>=10", make_test_node(NodeKind::Leq, 10, 1)),
            ("1+10", make_test_node(NodeKind::Add, 1, 10)),
            ("1-10", make_test_node(NodeKind::Sub, 1, 10)),
            ("1*10", make_test_node(NodeKind::Mul, 1, 10)),
            ("1/10", make_test_node(NodeKind::Div, 1, 10)),
            ("+1", Node::new_num(1)),
            ("-1", make_test_node(NodeKind::Sub, 0, 1)),
            (
                "2 * ( 3 + 4)",
                Node::new(
                    NodeKind::Mul,
                    Node::new_num(2),
                    make_test_node(NodeKind::Add, 3, 4),
                ),
            ),
            ("42", Node::new_num(42)),
        ];

        for (s, expected) in &tests {
            assert_eq!(
                expected,
                &expr(
                    &mut token::tokenize(s),
                    &mut Context::new(&HashMap::new(), &HashMap::new())
                )
                .unwrap()
            )
        }
    }

    #[test]
    fn test_stmt() {
        use crate::token;

        let tests = [(
            "int foo;",
            vec![Node::new_leaf(NodeKind::Declaration(make_int_dec("foo")))],
        )];

        for (s, expected) in &tests {
            let mut iter = token::tokenize(s);
            let mut actual = Vec::new();
            let mp1 = HashMap::new();
            let mp2 = HashMap::new();
            let ctx = &mut Context::new(&mp1, &mp2);
            while iter.peek() != None {
                actual.push(stmt(&mut iter, ctx).unwrap());
            }
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_unary() {
        use crate::token;
        let expected = vec![
            Node::new_num(1),
            Node::new_num(1),
            Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                make_int_dec("hoge"),
                8,
            )))),
            Node::new(NodeKind::Sub, Node::new_num(0), Node::new_num(1)),
            Node::new(
                NodeKind::Sub,
                Node::new_num(0),
                Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                )))),
            ),
            Node::new_unary(NodeKind::Deref, Node::new_num(1)),
            Node::new_unary(NodeKind::Addr, Node::new_num(1)),
            Node::new_unary(
                NodeKind::Deref,
                Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                )))),
            ),
            Node::new_unary(
                NodeKind::Addr,
                Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                )))),
            ),
            Node::new_unary(
                NodeKind::Deref,
                Node::new_unary(
                    NodeKind::Addr,
                    Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                        make_int_dec("hoge"),
                        8,
                    )))),
                ),
            ),
        ];

        let input = "1 +1 +hoge -1 -hoge *1 &1 *hoge &hoge *&hoge ";
        let iter = &mut token::tokenize(input);
        for i in expected {
            let mp1 = HashMap::new();
            let mp2 = HashMap::new();
            let ctx = &mut Context::new(&mp1, &mp2);
            ctx.lvar = Some(Rc::new(make_int_lvar("hoge", 8)));
            assert_eq!(i, unary(iter, ctx).unwrap());
        }

        let expected = vec![
            (Node::new_num(4), make_int_lvar("hoge", 8)),
            (Node::new_num(4), make_int_lvar("hoge", 8)),
            (Node::new_num(8), make_ptr_lvar("hoge", 8)),
            (Node::new_num(4), make_ptr_lvar("hoge", 8)),
        ];

        let input = "sizeof 1 sizeof (hoge) sizeof (hoge) sizeof(*hoge)";
        let iter = &mut token::tokenize(input);
        for i in expected {
            let mp1 = HashMap::new();
            let mp2 = HashMap::new();
            let ctx = &mut Context::new(&mp1, &mp2);
            ctx.lvar = Some(Rc::new(i.1));
            assert_eq!(i.0, unary(iter, ctx).unwrap());
        }
    }

    #[test]
    fn test_declaration() {
        use crate::token;
        use TypeKind::*;
        let tests = [
            (
                "int hoge",
                Declaration::new(BaseType::new(Int), Ident::new("hoge")),
            ),
            ("int *hoge", make_ptr_lvar("hoge", 8).dec),
            (
                "int **hoge",
                Declaration::new(
                    BaseType::new(Ptr(Rc::new(BaseType::new(Ptr(Rc::new(BaseType::new(
                        Int,
                    ))))))),
                    Ident::new("hoge"),
                ),
            ),
            (
                "int hoge[1]",
                Declaration::new(
                    BaseType::new(Array(1, Rc::new(BaseType::new(Int)))),
                    Ident::new("hoge"),
                ),
            ),
            (
                "int *hoge[1]",
                Declaration::new(
                    BaseType::new(Array(
                        1,
                        Rc::new(BaseType::new(Ptr(Rc::new(BaseType::new(Int))))),
                    )),
                    Ident::new("hoge"),
                ),
            ),
        ];

        for (input, expected) in &tests {
            assert_eq!(*expected, declaration(&mut token::tokenize(input)).unwrap());
        }
    }

    #[test]
    fn test_if() {
        use crate::token;
        let cond = Node::new(NodeKind::Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(15));
        let expected = make_if_node(cond, then);

        let input = "if ( 10 ==20 ) return 15;";
        let actual = stmt(
            &mut token::tokenize(input),
            &mut Context::new(&HashMap::new(), &HashMap::new()),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_if_else() {
        use crate::token;
        let cond = Node::new(NodeKind::Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(15));
        let els = Node::new_unary(
            NodeKind::Return,
            Node::new(NodeKind::Add, Node::new_num(10), Node::new_num(30)),
        );
        let expected = make_if_else_node(cond, then, els);

        let input = "if ( 10 ==20 ) return 15; else return 10+30;";
        let actual = stmt(
            &mut token::tokenize(input),
            &mut Context::new(&HashMap::new(), &HashMap::new()),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_while() {
        use crate::token;
        // Geqは左右を入れ替えてLeq
        let cond = Node::new(NodeKind::Leq, Node::new_num(20), Node::new_num(32));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(10));
        let expected = make_while_node(cond, then);

        let input = "while (32 >= 20 ) return 10;";
        let actual = stmt(
            &mut token::tokenize(input),
            &mut Context::new(&HashMap::new(), &HashMap::new()),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for() {
        use crate::token;

        let init = make_assign_node("i", 0, 8);
        let cond = Node::new(
            NodeKind::Lesser,
            Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            )))),
            Node::new_num(10),
        );
        let tmp_inc = Node::new(
            NodeKind::Add,
            Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            )))),
            Node::new_num(1),
        );
        let inc = Node::new(
            NodeKind::Assign,
            Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            )))),
            tmp_inc,
        );

        let ret = Node::new(
            NodeKind::Add,
            Node::new_leaf(NodeKind::Lvar(Rc::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            )))),
            Node::new_num(2),
        );
        let then = Node::new_unary(NodeKind::Return, ret);

        let expected = make_for_node(Some(init), Some(cond), Some(inc), then);

        let input = "for( i=0;i<10;i=i+1)return i+2;";
        let mp1 = HashMap::new();
        let mp2 = HashMap::new();
        let ctx = &mut Context::new(&mp1, &mp2);
        ctx.lvar = Some(Rc::new(make_int_lvar("i", 8)));
        let actual = stmt(&mut token::tokenize(input), ctx).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_block() {
        use crate::token;
        let input = "{1; 2; int hoge; hoge=4;}";
        let expected = vec![
            Node::new_num(1),
            Node::new_num(2),
            Node::new_leaf(NodeKind::Declaration(make_int_dec("hoge"))),
            make_assign_node("hoge", 4, 8),
        ];
        let expected = vec![Node::new_none(NodeKind::Block(expected))];
        let mut iter = token::tokenize(input);
        let mut actual = Vec::new();
        while iter.peek() != None {
            actual.push(
                stmt(
                    &mut iter,
                    &mut Context::new(&HashMap::new(), &HashMap::new()),
                )
                .unwrap(),
            );
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func() {
        use crate::token;
        let input = "add();";
        let expected_name = "add";
        let expected_args = vec![];
        let expected = make_fn_node(expected_name, expected_args);
        let mut mp2 = HashMap::new();
        mp2.insert(
            "add".to_string(),
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new("add"), Vec::new())),
        );

        let mut iter = token::tokenize(input);
        let actual = stmt(&mut iter, &mut Context::new(&HashMap::new(), &mp2)).unwrap();
        assert_eq!(expected, actual);

        let input = "three(1,2,3);";
        let expected_name = "three";
        let expected_args = vec![Node::new_num(1), Node::new_num(2), Node::new_num(3)];
        let expected = make_fn_node(expected_name, expected_args);
        let mut mp2 = HashMap::new();
        mp2.insert(
            "three".to_string(),
            Rc::new(FuncDef::new(
                TypeKind::Int,
                Ident::new("three"),
                vec![
                    Declaration::new(BaseType::new(TypeKind::Int), Ident::new("a")),
                    Declaration::new(BaseType::new(TypeKind::Int), Ident::new("a")),
                    Declaration::new(BaseType::new(TypeKind::Int), Ident::new("a")),
                ],
            )),
        );

        let mut iter = token::tokenize(input);
        let actual = stmt(&mut iter, &mut Context::new(&HashMap::new(), &mp2)).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func_def() {
        use crate::token;

        let expected_func_def =
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new("main"), Vec::new()));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(1))];
        let expected = Function::new(expected_func_def, None, 0, expected_nodes);

        let input = "{return 1;}";
        let func_def = Rc::new(FuncDef::new(TypeKind::Int, Ident::new("main"), vec![]));
        let iter = &mut token::tokenize(input);
        let actual = function(iter, func_def, &mut HashMap::new(), &mut HashMap::new()).unwrap();

        assert_eq!(expected, actual);

        let func_def = Rc::new(FuncDef::new(TypeKind::Int, Ident::new("main"), vec![]));
        let lvar1 = Lvar::new_leaf(make_int_dec("foo"), 8);
        let lvar2 = Lvar::new(lvar1.clone(), make_int_dec("bar"), 16);
        let expected_lvar = Rc::new(lvar2.clone());
        let node1 = Node::new_leaf(NodeKind::Declaration(make_int_dec("foo")));
        let node2 = make_assign_node("foo", 1, 8);
        let node3 = Node::new_leaf(NodeKind::Declaration(make_int_dec("bar")));
        let node4 = Node::new(
            NodeKind::Assign,
            Node::new_leaf(NodeKind::Lvar(Rc::new(lvar2.clone()))),
            Node::new_num(2),
        );
        let node5 = Node::new_unary(
            NodeKind::Return,
            Node::new(
                NodeKind::Add,
                Node::new_leaf(NodeKind::Lvar(Rc::new(lvar1))),
                Node::new_leaf(NodeKind::Lvar(Rc::new(lvar2))),
            ),
        );
        let expected_nodes = vec![node1, node2, node3, node4, node5];
        let expected = Function::new(func_def.clone(), Some(expected_lvar), 2, expected_nodes);

        let input = "{int foo;foo = 1; int bar;bar = 2; return foo+bar;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter, func_def, &mut HashMap::new(), &mut HashMap::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_param() {
        use crate::token;

        let expected = vec![make_int_dec("hoge")];

        let input = "int hoge";
        let iter = &mut token::tokenize(input);
        let actual = params(iter).unwrap();

        assert_eq!(expected, actual);

        let expected = vec![
            make_int_dec("foo"),
            make_int_dec("bar"),
            make_int_dec("hoge"),
        ];
        let input = "int foo,int bar,int hoge";
        let iter = &mut token::tokenize(input);
        let actual = params(iter).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fn_with_args() {
        use crate::token;

        let expected_func_def =
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new("main"), Vec::new()));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected = Function::new(expected_func_def, None, 0, expected_nodes);

        let expected_func_def =
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new("main"), Vec::new()));
        let input = "{return 0;}";
        let iter = &mut token::tokenize(input);
        let actual = function(
            iter,
            expected_func_def,
            &mut HashMap::new(),
            &mut HashMap::new(),
        )
        .unwrap();

        assert_eq!(expected, actual);

        let func_def = Rc::new(FuncDef::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![Declaration::new(
                BaseType::new(TypeKind::Int),
                Ident::new("foo"),
            )],
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected_lvar = Lvar::new_leaf(make_int_dec("foo"), 8);
        let expected = Function::new(func_def, Some(Rc::new(expected_lvar)), 1, expected_nodes);

        let expected_func_def = Rc::new(FuncDef::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![Declaration::new(
                BaseType::new(TypeKind::Int),
                Ident::new("foo"),
            )],
        ));
        let input = "{return 0;}";
        let iter = &mut token::tokenize(input);
        let actual = function(
            iter,
            expected_func_def,
            &mut HashMap::new(),
            &mut HashMap::new(),
        )
        .unwrap();

        assert_eq!(expected, actual);

        let expected_param = vec![
            make_int_dec("foo"),
            make_int_dec("bar"),
            make_int_dec("hoge"),
            make_int_dec("hey"),
        ];
        let expected_func_def = Rc::new(FuncDef::new(
            TypeKind::Int,
            Ident::new("main"),
            expected_param.clone(),
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected_lvar = Lvar::new(
            Lvar::new(
                Lvar::new(
                    Lvar::new_leaf(make_int_dec("foo"), 8),
                    make_int_dec("bar"),
                    16,
                ),
                make_int_dec("hoge"),
                24,
            ),
            make_int_dec("hey"),
            32,
        );
        let expected = Function::new(
            expected_func_def,
            Some(Rc::new(expected_lvar)),
            4,
            expected_nodes,
        );

        let input = "{return 0;}";
        let func_def = Rc::new(FuncDef::new(
            TypeKind::Int,
            Ident::new("main"),
            expected_param,
        ));
        let iter = &mut token::tokenize(input);
        let actual = function(iter, func_def, &mut HashMap::new(), &mut HashMap::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_primary() {
        use crate::token;
        let mut mp1 = HashMap::new();
        mp1.insert(
            "foo".to_string(),
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new("foo"), Vec::new())),
        );
        let mut mp2 = HashMap::new();
        mp2.insert(
            "foo".to_string(),
            Rc::new(FuncDef::new(
                TypeKind::Int,
                Ident::new("foo"),
                vec![Declaration::new(
                    BaseType::new(TypeKind::Int),
                    Ident::new("a"),
                )],
            )),
        );
        let mut mp3 = HashMap::new();
        mp3.insert(
            "foo".to_string(),
            Rc::new(FuncDef::new(
                TypeKind::Int,
                Ident::new("foo"),
                vec![
                    Declaration::new(BaseType::new(TypeKind::Int), Ident::new("a")),
                    Declaration::new(BaseType::new(TypeKind::Int), Ident::new("a")),
                ],
            )),
        );
        let tests = [
            ("1", Node::new_num(1), mp1.clone()),
            ("foo()", make_fn_node("foo", vec![]), mp1),
            ("foo(1)", make_fn_node("foo", vec![Node::new_num(1)]), mp2),
            (
                "foo(1,2)",
                make_fn_node("foo", vec![Node::new_num(1), Node::new_num(2)]),
                mp3,
            ),
        ];

        for (input, expected, mp) in &tests {
            let iter = &mut token::tokenize(input);
            assert_eq!(
                expected,
                &primary(iter, &mut Context::new(&HashMap::new(), &mp)).unwrap()
            );
        }
    }

    fn make_test_node(kind: NodeKind, lhs_num: u64, rhs_num: u64) -> Node {
        Node::new(kind, Node::new_num(lhs_num), Node::new_num(rhs_num))
    }

    fn make_assign_node(lhs: impl Into<String>, rhs: u64, offset: u64) -> Node {
        let mut node = Node::new_none(NodeKind::Assign);
        node.lhs = Some(Box::new(Node::new_leaf(NodeKind::Lvar(Rc::new(
            Lvar::new_leaf(make_int_dec(lhs.into()), offset),
        )))));
        node.rhs = Some(Box::new(Node::new_num(rhs)));
        node
    }

    fn make_if_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(NodeKind::If);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_if_else_node(cond: Node, then: Node, els: Node) -> Node {
        let mut node = make_if_node(cond, then);
        node.els = Some(Box::new(els));
        node
    }

    fn make_while_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(NodeKind::While);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_for_node(
        init: Option<Node>,
        cond: Option<Node>,
        inc: Option<Node>,
        then: Node,
    ) -> Node {
        let mut node = Node::new_none(NodeKind::For);
        node.init = init.map(|c| Box::new(c));
        node.cond = cond.map(|c| Box::new(c));
        node.inc = inc.map(|c| Box::new(c));
        node.then = Some(Box::new(then));
        node
    }

    fn make_fn_node(name: impl Into<String>, args: Vec<Node>) -> Node {
        let mut dec = vec![];
        for _ in 0..args.len() {
            dec.push(Declaration::new(
                BaseType::new(TypeKind::Int),
                Ident::new("a"),
            ));
        }
        Node::new_none(NodeKind::Func(
            Rc::new(FuncDef::new(TypeKind::Int, Ident::new(name), dec)),
            args,
        ))
    }

    fn make_int_dec(name: impl Into<String>) -> Declaration {
        Declaration::new(BaseType::new(TypeKind::Int), Ident::new(name.into()))
    }

    fn make_lvar(name: impl Into<String>, offset: u64, kind: TypeKind) -> Lvar {
        Lvar::new_leaf(
            Declaration::new(BaseType::new(kind), Ident::new(name)),
            offset,
        )
    }

    fn make_int_lvar(name: impl Into<String>, offset: u64) -> Lvar {
        make_lvar(name, offset, TypeKind::Int)
    }

    fn make_ptr_lvar(name: impl Into<String>, offset: u64) -> Lvar {
        make_lvar(
            name,
            offset,
            TypeKind::Ptr(Rc::new(BaseType::new(TypeKind::Int))),
        )
    }
}
