# simple parser to parse simple mysql

mysql 的词法分析和语法分析使用 flex 和 bison 生成
SQL 词法解析文件:

sql/sql_lex.h 、sql/lex_token.h 、sql/lex.h、 sql/lex_symbol.h、

sql/sql_lex.cc 、sql/ gen_lex_token.cc

SQL 语法解析文件:

sql/sql_yacc.yy 、sql/sql_yacc.cc、 sql/sql_yacc.h

SQL 语句的 hint 语法解析文件:

sql/sql_hints.yy 、sql/sql_hints.yy.cc

## Step1

CREATE TABLE table_name (column_name column_type,column_name column_type,...);

SELECT column_name,column_name
FROM table_name
[WHERE Clause];

=> L1 Grammer

statement => create_table_statement | select_statement

create_table_statement => CREATE TABLE table_name_tocken (column_name_tocken column_type_tocken,..., );

select_statement => SELECT column_name_tocken, ..., FROM table_name_tocken WHERE exprs_statement;

exprs_statement => expr_statement AND expr_statement OR expr_statement ...

exprs_statement => column_name_tocken = const_statement | column_name_tocken < const_statement | column_name_tocken > const_statement |...

const_statement => number | string_literals

很好的 rust parser 库
https://crates.io/crates/chumsky
