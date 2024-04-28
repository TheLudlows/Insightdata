

use anyhow::Result;
use opendal::services::{Fs};
use opendal::Operator;

#[tokio::test]
async fn main() -> Result<()> {
    // Create fs backend builder.
    let mut builder = Fs::default();
    // Set the root for fs, all operations will happen under this root.
    //
    // NOTE: the root must be absolute path.
    builder.root("/tmp");
    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();
    op.write("abc", "abc").await?;
    op.write_with("abc", "123").append(true).await?;
    let r = op.read_with("abc").range(0..123).await?;
    println!("{:?}", r);
    Ok(())
}