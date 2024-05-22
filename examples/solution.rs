/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023. All rights reserved.
 * Description: 上机编程认证
 */

use std::collections::{BTreeMap, HashMap, VecDeque};
struct Solution {
    tree: BTreeMap<String, u64>,
}





impl Solution {
    pub fn create() -> Self {
        Solution {
            tree: BTreeMap::default()
        }
    }
    pub fn add(&mut self, path: &str, count: u64) -> bool {
        let key = path.split("/").filter(|e| e.len() != 0). map(|e| e.to_string()).collect::<VecDeque<_>>();
        self.tree.add(key, count);
        // your code here
        true
    }

    pub fn query(&self, mut path: &str) -> u64 {
        let key = path.split("/").filter(|e| e.len() != 0). map(|e| e.to_string()).collect::<VecDeque<_>>();
        return  self.tree.get(key)
        // your code here
    }
}

// 以下为考题输入输出框架，此部分代码不建议改动；提交代码时请勿包含下面代码
mod io_formatter {
    use std::fs;
    use std::io::{stdin, stdout, BufRead, Result, Error, ErrorKind, BufReader};
    use std::str::FromStr;
    use serde::Serialize;
    use serde_json::{from_str, Serializer};

    struct OutFormatter {}

    impl serde_json::ser::Formatter for OutFormatter {
        fn begin_array_value<W: ?Sized + std::io::Write>(&mut self, writer: &mut W, first: bool) -> Result<()> {
            if !first { writer.write(b", ")?; }
            Ok(())
        }
    }

    pub fn normal_process() -> Result<()> {
        let mut line = String::new();
        //let mut file_in = BufReader::new(fs::OpenOptions::new().read(true).open("C:/Users/l00817341/Desktop/case/6.in")?);
        //let mut file_out = BufReader::new(fs::OpenOptions::new().read(true).open("C:/Users/l00817341/Desktop/case/6.out")?);

        let mut handle = stdin().lock();

        let mut read_count = handle.read_line(&mut line)?;

        //let mut read_count = file_in.read_line(&mut line)?;
        let name = line.trim();
        if name != "create" { return Err(Error::from(ErrorKind::InvalidData)); }
        let mut solution = super::Solution::create();
        println!("null");
        let mut out_string = String::new();
        //file_out.read_line(&mut out_string)?;
        let mut ser = Serializer::with_formatter(stdout(), OutFormatter {});
        while read_count != 0 {
            line.clear();
            out_string.clear();
            read_count = handle.read_line(&mut line)?;
            //file_out.read_line(&mut out_string)?;

            if line.trim().len() == 0 { continue; }
            let params = line.trim().split(" ");
            let param = params.map(|e| e.to_string()).collect::<Vec<_>>();
            let out = out_string.trim();
            if param.len() == 2 {
                let count = from_str(&param[1])?;
                solution.add(&param[0], count).serialize(&mut ser).unwrap();
                //assert_eq!(out, "true");
            } else if param.len() == 1 {
                solution.query(&param[0]).serialize(&mut ser).unwrap();
              /*  if n != from_str::<u64>(&out)? {
                    println!("{}", &param[0]);
                    return Ok(());
                }*/
            } else {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            println!();
        }

        Ok(())
    }
}

fn main() {
    if let Err(e) = io_formatter::normal_process() {
        eprint!("{}", e)
    }
}