use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use crate::types::DataType;

mod column;
mod table;

pub use self::column::*;
pub use self::table::*;

pub type SchemaId = u32;
pub type ColumnId = u32;
pub type TableId = u32;


#[derive(Clone)]
pub struct RootCatalog {
    inner: Arc<Mutex<Inner>>
}
#[derive(Debug, Clone)]
pub struct Inner {
    pub schemas: BTreeMap<SchemaId, SchemaCatalog>,
    pub schema_names: BTreeMap<String, SchemaId>,
    pub next_schema_id: SchemaId
}

#[derive(Debug, Clone)]
pub struct SchemaCatalog {
    pub name: String,
    pub id: SchemaId,
    pub tables: HashMap<TableId, TableCatalog>
}

#[derive(Debug, Clone)]
pub struct TableCatalog {
    pub name: String,
    pub id: TableId,
    pub tables: HashMap<ColumnId, Column>
}
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub id: ColumnId,
    pub col_type: DataType
}

#[derive(Debug, Clone)]
pub struct SchemaTableId {
    pub schema_id: SchemaId,
    pub table_id: TableId
}