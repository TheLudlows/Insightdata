use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use rand::{random, RngCore};

fn main() {
    let vec = vec![10 , 10000, 100000];


    for (i, x) in vec.iter().enumerate() {
        generate_in_out(i + 4, *x);
    }
}

fn generate_in_out(idx: usize, x: i32) {
    let path = PathBuf::from("d:/test_exe/");
    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    let file_in = path.join(idx.to_string() + ".in");
    let file_out = path.join(idx.to_string() + ".out");

    if file_in.exists() {
        fs::remove_file(&file_in).unwrap();
    }
    if file_out.exists() {
        fs::remove_file(&file_out).unwrap();
    }
    let mut file_in = OpenOptions::new().read(true).write(true).create(true).open(file_in).unwrap();
    let mut file_out = OpenOptions::new().read(true).write(true).create(true).open(file_out).unwrap();

    let mut out = vec![];
    let mut input = vec![];

    let x = (x as f64).powf(1.0 / 3.0) as u32;
    println!(" {}", x);

    let path_root = "/".to_string();

    let mut path1_vec = vec![];
    let mut path2_vec = vec![];
    let mut path3_vec = vec![];
    let mut total = 0;
    for i in 0..x {
        let mut path1 = "/a".to_string() + &i.to_string();
        let mut count1 = 0;
        for j in 0..x {
            let path2 = path1.clone() + "/s" + &j.to_string();
            let mut count2 = 0;
            for k in 0..x {
                let path3 = path2.clone() + "/o" + &k.to_string();
                let count = rand::thread_rng().next_u32() % 100;
                path3_vec.push((path3.clone(), count));
                count2 += count;
            }
            path2_vec.push((path2.clone(), count2));
            count1 += count2;
        }
        path1_vec.push((path1.clone(), count1));
        total += count1;
    }


    // write all
    for (p, c) in path3_vec.iter() {
        let mut line = p.to_string();
        line.push_str(" ");
        line.push_str(&c.to_string());
        input.push(line);
        out.push("true".to_string());
    }

    // read all
    for (p, c) in path3_vec.iter() {
        let line = p.to_string();
        input.push(line);
        out.push(c.to_string());
    }

    // read level 2
    for (p, c) in path2_vec.iter() {
        let line = p.to_string();
        input.push(line);
        out.push(c.to_string());
    }

    // read level 1
    for (p, c) in path1_vec.iter() {
        let line = p.to_string();
        input.push(line);
        out.push(c.to_string());
    }

    // root count

    input.push(path_root);
    out.push(total.to_string());

    file_in.write_all("create\n".as_bytes()).unwrap();
    for line in input.iter() {
        file_in.write_all(format!("{}\n", line).as_bytes()).unwrap();
    }
    file_out.write_all("null\n".as_bytes()).unwrap();
    for line in out.iter() {
        file_out.write_all(format!("{}\n", line).as_bytes()).unwrap();
    }
}
