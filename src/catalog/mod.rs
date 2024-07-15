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


pub struct RootCatalog {
    inner: Mutex<Inner>
}

impl RootCatalog {
    pub fn get_table_by_id(&self, id: &SchemaTableId) -> Option<Arc<TableCatalog>> {
        let root = self.inner.lock().unwrap();
        let schema_catalog = root.schemas.get(&id.schema_id)?;
        schema_catalog.get_table(&id.table_id)
    }
    pub fn get_schema_by_id(&self,id: SchemaId) -> Option<Arc<SchemaCatalog>> {
        self.inner.lock().unwrap().schemas.get(&id).cloned()
    }
}
#[derive(Debug, Clone)]
pub struct Inner {
    pub schemas: BTreeMap<SchemaId, Arc<SchemaCatalog>>,
    pub schema_names: BTreeMap<String, SchemaId>,
    pub next_schema_id: SchemaId
}

#[derive(Debug, Clone)]
pub struct SchemaCatalog {
    pub name: String,
    pub id: SchemaId,
    pub tables: HashMap<TableId, Arc<TableCatalog>>
}

impl SchemaCatalog {
    pub fn get_table(&self, table_id: &TableId) -> Option<Arc<TableCatalog>>{
        self.tables.get(table_id).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct TableCatalog {
    pub name: String,
    pub id: SchemaTableId,
    pub tables: HashMap<ColumnId, Arc<Column>>
}
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub id: ColumnId,
    pub col_type: DataType
}

#[derive(Debug, Clone, Copy)]
pub struct SchemaTableId {
    pub schema_id: SchemaId,
    pub table_id: TableId
}