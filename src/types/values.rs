use serde::Serialize;
use crate::types::{Bytes, Date};
use parse_display::Display;
#[derive(Clone, Debug, Serialize, Display)]
pub enum DataValue {
    // NOTE: Null comes first.
    // => NULL is less than any non-NULL values
    #[display("null")]
    Null,
    #[display("{0}")]
    Bool(bool),
    #[display("{0}")]
    Int(i64),
    #[display("{0}")]
    Float64(f64),
    #[display("'{0}'")]
    String(String),
    #[display("{0}")]
    Bytes(Bytes),
    #[display("{0}")]
    Date(Date),
}