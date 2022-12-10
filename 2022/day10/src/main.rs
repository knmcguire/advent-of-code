use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read in a input file, of which the name is passed in as a command line argument
    let args: Vec<String> = std::env::args().collect();
    let file_str = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file_str).lines();



    // Loop through the lines of the file and retrieve the string
    let mut vec_str : Vec<String> = Vec :: new();
    for line in reader {
        let line_str = line.unwrap();
        if line_str.contains("noop") {
            vec_str.push("noop".to_string());
        } else {
            vec_str.push("noop".to_string());
            let mut instruction = line_str.clone();
            vec_str.push(instruction);
        }
    }
    let mut x : i32 = 1;
    let mut cycle_num = 1;
    let mut sum = 0;
    for line in vec_str.iter() {
        if line.contains("noop") {
            cycle_num += 1;
        } else {
            // retrieve last number of string including the sign
            let number : i32 = line.split_whitespace().last().unwrap().parse().unwrap();
            x += number;
            cycle_num+= 1;
        }
        let array = [20, 60, 100, 140, 180, 220];
        // check if cycle_num is in array
        if array.contains(&cycle_num) {
            println!("cycle_num: {}, x: {}, {}", cycle_num, x, cycle_num*x);
            sum += cycle_num*x
        }
        println!("cycle_num: {}, x: {}", cycle_num, x);
    }
    println!("sum: {}", sum);

}
