use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;
use crate::catalog::{RootCatalog, SchemaTableId, TableCatalog};
use crate::storage::Storage;

pub struct MemStorage {
    root_catalog: RootCatalog,
    tables: Mutex<HashMap<SchemaTableId, MemTable>>
}
pub struct MemTable {

}
impl Storage for MemStorage {
    type TableType = MemTable;

    fn create_table(&self, table: &TableCatalog) -> impl Future<Output=anyhow::Result<()>> {
        todo!()
    }

    fn get_table(&self, table_id: SchemaTableId) -> impl Future<Output=anyhow::Result<Self::TableType>> {
        todo!()
    }

    fn drop_table(&self, table_id: SchemaTableId) -> impl Future<Output=anyhow::Result<Self::TableType>> {
        todo!()
    }
}