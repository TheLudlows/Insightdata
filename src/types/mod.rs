mod types;
mod values;

use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use serde::Serialize;
pub use types::DataType;
pub use values::*;

pub const UNIX_EPOCH_DAYS: i32 = 719_163;

#[derive(Clone, Debug, Serialize)]
pub struct  Bytes(Vec<u8>);
#[derive(Clone, Debug, Serialize)]
pub struct Date(i32);

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            NaiveDate::from_num_days_from_ce_opt(self.0 + UNIX_EPOCH_DAYS)
                .unwrap()
                .format("%Y-%m-%d")
        )
    }
}
