
use std::fs::File;
use parquet::column::page::Page;
use parquet::file::reader::{FileReader, SerializedFileReader};

#[test]
fn main() {
    // 打开Parquet文件
    let file = File::open("example.parquet").unwrap();
    let reader = SerializedFileReader::new(file).unwrap();

    // 获取文件元数据
    let metadata = reader.metadata();
    let file_metadata = metadata.file_metadata();

    println!("Parquet Version: {}", file_metadata.version());
    println!("Created By: {}", file_metadata.created_by().unwrap_or("Unknown"));

    // 遍历所有行组
    for row_group in 0..metadata.num_row_groups() {
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
                    },
                    Page::DictionaryPage { buf, num_values, encoding, .. } => {
                        println!("    Dictionary Page:");
                        println!("      Num Values: {}", num_values);
                        println!("      Encoding: {:?}", encoding);
                    },
                    _ => {
                        println!("    Other Page Type");
                    }
                }
            }
        }
    }
}
