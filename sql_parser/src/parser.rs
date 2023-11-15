use std::{fmt::Error, process::id};

use chumsky::prelude::*;

#[derive(Debug)]
pub struct Column {
    name: String,   // 列名
    v_type: String, // 列类型
    is_primary_key: bool,
}

#[derive(Debug)]
pub struct Selector {
    column_name: String,   // 列名
    alias: Option<String>, // 别名
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Num(i32),
    Var(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Statement {
    Select {
        table_name: String,
        selectors: Vec<Selector>,
        where_condition: Box<Expr>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<Column>,
    },
}

fn gen_parser_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    // 解析变量名
    let ident = text::ident::<char, Simple<char>>().padded().map(Expr::Var);
    let int = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));

    let expr = recursive(|expr| {
        let atom = int
            .padded()
            .or(expr.delimited_by(just('('), just(')')))
            .or(ident);

        let op = |c: &'static str| just(c).padded();

        let unary = op("-")
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                choice((
                    op("*").to(Expr::Mul as fn(_, _) -> _),
                    op("/").to(Expr::Div as fn(_, _) -> _),
                ))
                .then(unary)
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                choice((
                    op("+").to(Expr::Add as fn(_, _) -> _),
                    op("-").to(Expr::Sub as fn(_, _) -> _),
                ))
                .then(product)
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let equal = sum
            .clone()
            .then(
                choice((op("==").to(Expr::Equal), op("=").padded().to(Expr::Equal)))
                    .then(sum)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
        equal
    });

    expr
}

fn gen_parser_statement() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_expr() {
        let parser_expr = gen_parser_expr();
        assert_eq!(
            Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2))),
            parser_expr.parse("   1 + 2").unwrap()
        );
        assert_eq!(
            Expr::Equal(
                Box::new(Expr::Neg(Box::new(Expr::Num(99)))),
                Box::new(Expr::Num(2))
            ),
            parser_expr.parse("-99 == 2").unwrap()
        );
    }
}
