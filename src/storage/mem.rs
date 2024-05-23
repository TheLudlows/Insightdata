use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use crate::catalog::{Column, ColumnId, RootCatalog, SchemaTableId, TableCatalog};
use crate::storage::{ScanOptions, Storage, Table, Transaction, TxnIterator};
use anyhow::Result;
use arrow::array::RecordBatch;

pub struct MemStorage {
    root_catalog: Arc<RootCatalog>,
    tables: Mutex<HashMap<SchemaTableId, MemTable>>
}
pub struct MemTable {
    pub schema_table_id: SchemaTableId,
    pub columns: Arc<Column>,
    pub data: MemTableData
}
pub struct MemTableData {
    bench: Vec<RecordBatch>
}
impl Table for MemTable {
    type Transaction = MemTableTxn;

    fn columns(&self) -> Result<Arc<[Column]>> {
        todo!()
    }

    async fn write(&self) -> Result<Self::Transaction> {
        todo!()
    }

    async fn read(&self) -> Result<Self::Transaction> {
        todo!()
    }

    async fn update(&self) -> Result<Self::Transaction> {
        todo!()
    }

    fn table_id(&self) -> SchemaTableId {
        todo!()
    }
}

impl Storage for MemStorage {
    type TableType = MemTable;

    async fn create_table(&self, table: &TableCatalog) -> Result<()> {
        self.root_catalog.get_table_by_id(&table.id).ok_or_else(||)
        let tables = self.tables.lock();
    }

    async fn get_table(&self, table_id: SchemaTableId) ->Result<Self::TableType> {
        todo!()
    }

    async fn drop_table(&self, table_id: SchemaTableId) -> Result<Self::TableType> {
        todo!()
    }
}
pub struct MemTableTxn {
    
}

impl Transaction for MemTableTxn {
    type TxnIteratorType = MemTxnIterator;

    async fn scan(&self, col_idx: &[ColumnId], options: ScanOptions) -> Result<Self::TxnIteratorType>{
        todo!()
    }

    async fn append(&mut self, columns: RecordBatch) -> Result<()> {
        todo!()
    }

    async fn delete(&mut self) -> Result<()> {
        todo!()
    }

    async fn commit(self) -> Result<()> {
        todo!()
    }

    async fn abort(self) -> Result<()> {
        todo!()
    }
}
pub struct MemTxnIterator {

}

impl TxnIterator for MemTxnIterator {

}