
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use anyhow::Result;
fn main() -> Result<()> {
    let res = parse("select * from tablea where a = 1; select *from tableb");
    if res.is_err() {
        println!("{:?}", res.err())
    } else {
        for stat in res.unwrap() {
            println!("{}", stat)
        }
    }
    Ok(())
}
pub fn parse(sql: impl Into<String>) -> Result<Vec<Statement>, ParserError> {
    let dialect = PostgreSqlDialect{};
    Parser::parse_sql(&dialect, &sql.into())
}
