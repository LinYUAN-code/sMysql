use chumsky::{
    prelude::*,
    text::{ident, keyword},
};

#[derive(Debug)]
pub struct Column {
    name: String,   // 列名
    v_type: String, // 列类型
    is_primary_key: bool,
    is_not_null: bool,
    auto_increment: bool,
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
    StringLiteral(String),
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
        where_conditions: Option<Vec<Expr>>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<Column>,
    },
    Insert {
        table_name: String,
        field_names: Vec<String>,
        values: Vec<Expr>,
    },
}

fn gen_parser_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    let op = |c: &'static str| just(c).padded();
    let ident = text::ident::<char, Simple<char>>().padded().map(Expr::Var);
    let int = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));
    let string_literal = one_of::<_, _, Simple<char>>("\"'")
        .ignore_then(none_of("\"'").repeated())
        .then_ignore(one_of("\"'"))
        .collect::<String>()
        .map(|s| Expr::StringLiteral(s));
    let expr = recursive(|expr| {
        let atom = int
            .padded()
            .or(expr.delimited_by(just('('), just(')')))
            .or(ident)
            .or(string_literal);

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

fn gen_parser_select_statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    let keyword = |s: &'static str| keyword(s).or(keyword(s.to_lowercase())).padded();
    let selector = keyword("FROM")
        .not()
        .then(ident().map(|s: String| s))
        .padded()
        .map(|(a, mut name)| -> Selector {
            name.insert(0, a);
            Selector {
                column_name: name,
                alias: None,
            }
        })
        .then(keyword("AS").then(ident()).or_not())
        .then_ignore(just(',').or_not())
        .map(|(mut selector, alias)| {
            match alias {
                Some(val) => selector.alias = Some(val.1),
                None => (),
            }
            selector
        });
    let expr_parser = gen_parser_expr();
    let where_parser = keyword("WHERE")
        .then(expr_parser.repeated())
        .map(|((), exprs)| exprs);

    let select_statement = keyword("SELECT")
        .then(selector.repeated())
        .map(|((), selectors)| selectors)
        .then(keyword("FROM"))
        .then(ident().padded())
        .then(where_parser.or_not())
        .then(just(";").padded().ignored())
        .map(
            |((((selectors, _), table_name), where_conditions), _)| Statement::Select {
                table_name,
                selectors,
                where_conditions,
            },
        );
    select_statement
}

fn gen_parser_create_table_statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    let just_keyword = |s: String| just(s.clone()).or(just(s.to_lowercase())).padded();
    let keyword = |s: &'static str| keyword(s).or(keyword(s.to_lowercase())).padded();

    // todo! 这里NOT NULL AUTO_INCREMENT的设置必须要按照顺序  怎么支持不顺序设置
    let column_parser = ident()
        .padded()
        .then(ident().padded())
        .then(just_keyword("NOT NULL".to_string()).or_not())
        .then(just_keyword("AUTO_INCREMENT".to_string()).or_not())
        .then(just_keyword("PRIMARY KEY".to_string()).or_not())
        .map(
            |((((column_name, v_type), is_not_null), auto_increment), is_primary_key)| Column {
                name: column_name,
                v_type,
                is_primary_key: is_primary_key.is_some(),
                is_not_null: is_not_null.is_some(),
                auto_increment: auto_increment.is_some(),
            },
        )
        .then(just(",").or_not().padded())
        .map(|(column, _)| column);

    let create_table_statement = keyword("CREATE")
        .then(keyword("TABLE"))
        .then(ident().map(|s| s))
        .map(|((_, _), table_name)| table_name)
        .then(just("(").padded().ignored())
        .then(column_parser.repeated().map(|columns| columns))
        .then(just(")").padded().ignored())
        .then(just(";").padded().ignored())
        .map(
            |((((table_name, _), columns), _), _)| Statement::CreateTable {
                table_name,
                columns,
            },
        );
    create_table_statement
}

fn gen_parser_insert_statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    let just_keyword = |s: String| just(s.clone()).or(just(s.to_lowercase())).padded();

    let insert_statement = just_keyword("INSERT INTO".to_string())
        .then(ident().map(|s| s))
        .map(|(_, table_name)| table_name)
        .then(just("(").padded().ignored())
        .then(
            ident()
                .padded()
                .then(just(",").or_not().ignored())
                .repeated()
                .map(|arr| -> Vec<String> { arr.into_iter().map(|value| value.0).collect() }),
        )
        .then(just(")").padded().ignored())
        .map(|(((table_name, _), field_names), _)| (table_name, field_names))
        .then(just_keyword("VALUES".to_string()).ignored())
        .then(just("(").padded().ignored())
        .then(
            gen_parser_expr()
                .padded()
                .then(just(",").or_not().ignored())
                .repeated()
                .map(|arr| -> Vec<Expr> { arr.into_iter().map(|value| value.0).collect() }),
        )
        .then(just(")").padded().ignored())
        .then(just(";"))
        .map(
            |((((((table_name, field_names), _), _), values), _), _)| Statement::Insert {
                table_name,
                field_names,
                values,
            },
        );
    insert_statement
}

pub fn gen_parser() -> impl Parser<char, Statement, Error = Simple<char>> {
    gen_parser_create_table_statement()
        .or(gen_parser_insert_statement())
        .or(gen_parser_select_statement())
        .then_ignore(end())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_select_statement() {
        println!(
            "{:?}",
            gen_parser_select_statement()
                .parse("SELECT name, age, hobby as like, talent FROM user WHERE age = 1;")
        );
        println!(
            "{:?}",
            gen_parser_select_statement()
                .parse("select name, age, hobby as like, talent from user where age = 1;")
        );
    }

    #[test]
    fn test_create_table_statement() {
        println!(
            "{:?}",
            gen_parser_create_table_statement()
                .parse("create table user(id string PRIMARY KEY,name string NOT NULL, age int32 NOT NULL, hobby string, talent string);")
        );
    }

    #[test]
    fn test_insert_statement() {
        println!(
            "{:?}",
            gen_parser_insert_statement()
                .parse("INSERT INTO user (id, name, age, hobby, talent) VALUES ('1', 'linYuan', 3, 'play Game','code');")
        );
    }

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
        assert_eq!(
            Expr::StringLiteral("hello world".to_string()),
            parser_expr.parse("'hello world'").unwrap()
        );
    }
}
