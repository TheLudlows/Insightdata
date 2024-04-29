use std::future::Future;
use anyhow::Result;
use sqlparser::ast::Table;
use crate::catalog::{ColumnCatalog, ColumnId, TableId};

mod dal;

pub trait Storage: Sync + Send{

    async fn create_table(
        &self,
        table_id: TableId,
        table_name: &str,
        columns: &[ColumnCatalog],
        pk_ids: &[ColumnId],
    ) -> Result<()>;

    async fn get_table(&self, table_id: TableId) -> Result<Table>;

    async fn drop_table(&self, table_id: TableId) -> Result<Table>;
}