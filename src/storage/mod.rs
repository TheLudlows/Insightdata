use std::collections::Bound;
use std::future::Future;
use std::sync::Arc;
use anyhow::Result;
use arrow::array::RecordBatch;
use serde::Serialize;
use crate::catalog::{Column, ColumnId, SchemaTableId, TableCatalog, TableId};
use crate::types::DataValue;

mod dal;
mod mem;

pub trait Storage: Sync + Send {
    type TableType: Table;
    fn create_table(&self, table: &TableCatalog) -> impl Future<Output=Result<()>>;

    fn get_table(&self, table_id: SchemaTableId) -> impl Future<Output=Result<Self::TableType>>;

    fn drop_table(&self, table_id: SchemaTableId) -> impl Future<Output=Result<Self::TableType>>;
}

pub trait Table {

    type Transaction: Transaction;

    /// Get schema of the current table
    fn columns(&self) -> Result<Arc<[Column]>>;

    /// Begin a read-write-only txn
    fn write(&self) -> impl Future<Output = Result<Self::Transaction>>;

    /// Begin a read-only txn
    fn read(&self) -> impl Future<Output = Result<Self::Transaction>> ;

    /// Begin a txn that might delete or update rows
    fn update(&self) -> impl Future<Output = Result<Self::Transaction>>;

    /// Get table id
    fn table_id(&self) -> SchemaTableId;

}

pub trait Transaction: Sync + Send{
    /// Type of the table iterator
    type TxnIteratorType: TxnIterator;

    fn scan(&self, col_idx: &[ColumnId], options: ScanOptions) -> impl Future<Output = Result<Self::TxnIteratorType>> + Send;

    /// Append data to the table. Generally, `columns` should be in the same order as
    /// [`ColumnCatalog`] when constructing the [`Table`].
    fn append(&mut self, columns: RecordBatch) -> impl Future<Output = Result<()>> + Send;

    /// Delete a record.
    fn delete(&mut self) -> impl Future<Output = Result<()>>;

    /// Commit a transaction.
    fn commit(self) -> impl Future<Output = Result<()>>;

    /// Abort a transaction.
    fn abort(self) -> impl Future<Output = Result<()>>;
}

#[derive(Debug, Default)]
pub struct ScanOptions {
    is_sorted: bool,
    reversed: bool,
    filter: Option<KeyRange>,
}
#[derive(Debug, Clone, Serialize)]
pub struct KeyRange {
    /// Start bound.
    pub start: Bound<DataValue>,
    /// End bound.
    pub end: Bound<DataValue>,
}

pub trait TxnIterator {

}