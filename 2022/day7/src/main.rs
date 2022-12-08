use std::fs::{File, remove_dir_all, remove_file, create_dir};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::process::Command;
use std::io::prelude::*;
use std::path::PathBuf;



fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let mut file_str = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file_str).lines();

    let file_path = PathBuf::from("target/test_dir/output.txt");
    create_dir("target/test_dir").unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(b"").unwrap();

    for line in reader {
        let line_str = line.unwrap();

        if line_str.contains("$ ") {

            println!(" {:?}", line_str);
        }else if line_str.contains("dir ") {

            println!("{:?}", line_str);
        }else {
            println!("{:?}", line_str);

        }
    }
    remove_dir_all("target/test_dir").unwrap();
}
