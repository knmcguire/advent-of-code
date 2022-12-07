use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::process::Command;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let mut file_str = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file_str).lines();

    let mut dir_map: HashMap<String, u32> = HashMap::new();
    let mut dir_name = String::new();
    let mut dir_size: u32 = 0;

    for line in reader {
        let line_str = line.unwrap();
        let mut line_tmp = line_str.clone();

        if line_str.contains("$ ") {

            println!(" {:?}", line_tmp);
        }else if line_str.contains("dir ") {
            println!("{:?}", dir_name);
            dir_map.insert(dir_name, dir_size);
            dir_name = line_tmp.replace("dir ", "");
            dir_size = 0;
            println!("{:?}", dir_name);
        }else {
            let file_vec = line_tmp.split(" ").collect::<Vec<&str>>();

            dir_size += file_vec[0].parse::<u32>().unwrap();
        }
    }
    println!("{:?}", dir_map);
}
