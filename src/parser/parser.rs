use datafusion::sql::parser::DFParser;
use sqlparser::ast::Statement;
use anyhow::Result;
#[test]
fn test_parser() {
    let r = DFParser::parse_sql("select * from a").unwrap();
    println!("{:?}", r);
}