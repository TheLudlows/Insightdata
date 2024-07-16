use std::sync::Arc;

use anyhow::Result;
use arrow::array::{Int32Array, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use bytes::Bytes;
use parquet::arrow::arrow_reader::{ArrowReaderBuilder, ArrowReaderOptions, ParquetRecordBatchReaderBuilder};
use parquet::arrow::ArrowWriter;

use crate::storage::dal::{read, write_to};

#[tokio::test]
async fn main() -> Result<()> {
    let buf = read("test_pq").await?;
   let builder =  ParquetRecordBatchReaderBuilder::try_new();
    ArrowReaderOptions::new().rea
    // 获取文件元数据
    /* let metadata = reader.metadata();
     let file_metadata = metadata.file_metadata();

     println!("Parquet Version: {}", file_metadata.version());
     println!("Created By: {}", file_metadata.created_by().unwrap_or("Unknown"));*/

    // 遍历所有行组
    /*for row_group in 0..metadata.num_row_groups() {
        let row_group_reader = reader.get_row_group(row_group).unwrap();
        let row_group_metadata = row_group_reader.metadata();

        println!("Row Group {}:", row_group);

        // 遍历所有列块
        for col in 0..row_group_metadata.num_columns() {
            let column_chunk = row_group_reader.get_column_reader(col).unwrap();
            let column_chunk_metadata = row_group_metadata.column(col);

            println!("  Column Chunk {}: {}", col, column_chunk_metadata.column_path().string());
            println!("    Compression: {:?}", column_chunk_metadata.compression());

            // 遍历所有页面
            let mut page_reader = column_chunk.get_page_reader().unwrap();
            while let Some(page) = page_reader.get_next_page().unwrap() {
                match page {
                    Page::DataPage {
                        buf,
                        num_values,
                        encoding,
                        ..
                    } => {
                        println!("    Data Page:");
                        println!("      Num Values: {}", num_values);
                        println!("      Encoding: {:?}", encoding);
                    }
                    Page::DictionaryPage { buf, num_values, encoding, .. } => {
                        println!("    Dictionary Page:");
                        println!("      Num Values: {}", num_values);
                        println!("      Encoding: {:?}", encoding);
                    }
                    _ => {
                        println!("    Other Page Type");
                    }
                }
            }
        }
    }*/
}
#[tokio::test]
async fn test_write() -> Result<()> {
    // 创建Schema
    let schema = Arc::new(
        Schema::new(vec![
            Field::new("id", DataType::Int32, true),
            Field::new("name", DataType::Utf8, true),
        ])
    );

    // 创建RecordBatch
    let id_array = Int32Array::from_iter_values([1, 2, 3]);
    let name_array = StringArray::from_iter_values(&["Alice", "Bob", "Charlie"]);
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(id_array), Arc::new(name_array)],
    )?;
    let mut buffer = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)?;
        writer.write(&batch)?;
    }

    write_to("test_pq", Bytes::from(buffer)).await?;
    Ok(())
}
