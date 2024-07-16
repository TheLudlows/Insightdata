use arrow::datatypes::{DataType, Field, Schema};
use datafusion::parquet::arrow::ArrowWriter;

#[test]
fn test_arrow() {
    let schema = Schema::new(vec![
        Field::new("int_values", DataType::Int32, false),
        Field::new("float_values", DataType::Float64, false),
    ]);


}