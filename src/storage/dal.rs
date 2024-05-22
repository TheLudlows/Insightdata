use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashMap};
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

#[test]
fn test() {
    let num1 = vec![1, 2, 3, 4, 5];
    let num2 = vec![4, 5, 6, 7, 8];
    let result = intersection(&num1, &num2);
    assert_eq!(result, vec![4, 5]);
}

fn intersection(n1: &[i32], n2: &[i32]) -> Vec<i32> {
    //n1.iter().filter(|&x| n2.contains(x)).cloned().collect()
    /*let mut result = n1.to_vec();
    result.retain(|x| n2.contains(x));
    result*/

    n1.iter().skip_while(|&x| !n2.contains(x)).cloned().collect()
   /* let mut result = n1.to_vec();
    result.extend(n2);
    result*/
}


