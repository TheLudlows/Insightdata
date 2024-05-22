use std::future::Future;
use anyhow::Result;
use crate::catalog::{SchemaTableId, TableCatalog, TableId};

mod dal;
mod mem;

pub trait Storage: Sync + Send {
    type TableType: Table;
    fn create_table(&self, table: &TableCatalog) -> impl Future<Output=Result<()>>;

    fn get_table(&self, table_id: SchemaTableId) -> impl Future<Output=Result<Self::TableType>>;

    fn drop_table(&self, table_id: SchemaTableId) -> impl Future<Output=Result<Self::TableType>>;
}

pub trait Table {

}