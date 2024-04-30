use std::future::Future;
use anyhow::Result;
use sqlparser::ast::Table;
use crate::catalog::{ColumnCatalog, ColumnId, TableId};

mod dal;

pub trait Storage: Sync + Send {
    fn create_table(
        &self,
        table_id: TableId,
        table_name: &str,
        columns: &[ColumnCatalog],
        pk_ids: &[ColumnId],
    ) -> impl Future<Output=Result<()>>;

    fn get_table(&self, table_id: TableId) -> impl Future<Output=Result<Table>>;

    fn drop_table(&self, table_id: TableId) -> impl Future<Output=Result<Table>>;
}