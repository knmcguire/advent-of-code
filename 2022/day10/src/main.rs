use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read in a input file, of which the name is passed in as a command line argument
    let args: Vec<String> = std::env::args().collect();
    let file_str = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file_str).lines();

    let mut x : i32 = 1;
    let mut cycle_num = 0;
    // Loop through the lines of the file and retrieve the string
    for line in reader {
        let line_str = line.unwrap();
        if line_str.contains("noop") {
            cycle_num += 1;
        } else {
            // retrieve last number of string including the sign
            let number : i32 = line_str.split_whitespace().last().unwrap().parse().unwrap();
            x += number;
            cycle_num+= 2;
        }
        println!("cycle_num: {}, x: {}", cycle_num, x);
    }

}
