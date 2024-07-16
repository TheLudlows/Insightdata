
use anyhow::Result;
use bytes::Bytes;
use opendal::services::{Fs};
use opendal::{Builder, Operator};

pub async fn write_to(path: &str, buf: Bytes) -> Result<()> {
    // Create fs backend builder.
    let mut builder = Fs::default();
    // Set the root for fs, all operations will happen under this root.
    //
    // NOTE: the root must be absolute path.
    builder.root("d:/test_data");
    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();
    op.write(path, buf).await?;
    Ok(())
}

pub async fn read(path: &str) -> Result<Vec<u8>> {
    // Create fs backend builder.
    let mut builder = Fs::default();
    // Set the root for fs, all operations will happen under this root.
    //
    // NOTE: the root must be absolute path.
    builder.root("d:/test_data");
    let op: Operator = Operator::new(builder)?.finish();
    let vec = op.read(path).await?;
    Ok(vec)
}


