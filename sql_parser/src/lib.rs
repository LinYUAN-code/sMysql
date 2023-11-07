use lexer::Lexer;

pub mod lexer;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Statement {}

struct CreateTableStatement {
    table_name: String,
    column_defs: Vec<(String, String)>,
}

impl Statement for CreateTableStatement {}

struct ConstLiteral {
    number_literal: Option<i32>,
    string_literal: Option<String>,
}
struct Expr {
    column: String,
    op: String,
    const_statement: ConstLiteral,
}
struct SelectStatement {
    table_name: String,
    column_names: Vec<String>,
    where_conditions: Vec<Expr>,
}

impl Statement for SelectStatement {}

pub fn parse(source: String) -> Vec<Box<dyn Statement>> {
    vec![]
}

pub fn parse_statement(lexer: Lexer) -> Box<dyn Statement> {
    Box::new(CreateTableStatement {
        table_name: todo!(),
        column_defs: todo!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
