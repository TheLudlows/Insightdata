use std::collections::HashMap;


mod column;
mod table;

pub use self::column::*;
pub use self::table::*;

pub type TableId = u32;
pub type ColumnId = u32;
#[derive(Debug, Clone)]
pub struct RootCatalog {
    pub tables: HashMap<TableId, ()>,
}