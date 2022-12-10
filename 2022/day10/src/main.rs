use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read in a input file, of which the name is passed in as a command line argument
    let args: Vec<String> = std::env::args().collect();
    let file_str = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file_str).lines();

    // read in the file and add the instructions to a vector at the right cycles
    let mut vec_str : Vec<String> = Vec :: new();
    for line in reader {
        let line_str = line.unwrap();
        if line_str.contains("noop") {
            vec_str.push("noop".to_string());
        } else {
            let instruction = line_str.clone();
            vec_str.push("noop".to_string());
            vec_str.push(instruction.clone());
        }
    }

    // initialize variables for entire exercise
    let mut cycle_num = 1;

    // initialize variables part 1
    let mut x : i32 = 1;
    let mut sum = 0;
    let array = [20, 60, 100, 140, 180, 220];

    // initialize variables part 2
    let mut crt : Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    let mut position_sprite = 1;

    // iterate over the vector and execute the instructions
    for line in vec_str.iter() {
        // Find the current index and level of the CRT
        let current_index = ((cycle_num-1) % 40 ) as usize;
        let crt_level = ((cycle_num-1)  / 40 )as usize;

        //if current index is similar or 1 off from position_sprite, than draw '#'
        // or else draw '.'
        let mut current_drawn_character = '.';
        if current_index == position_sprite as usize || current_index == (position_sprite - 1) as usize || current_index == (position_sprite + 1) as usize {
            current_drawn_character = '#';
        }
        crt[crt_level][current_index] = current_drawn_character;

        // execute the instruction if it is in the line
        // with the exception of noop
        if line.contains("noop") {
            // do nothing
        } else {
            // retrieve last number of string including the sign
            let number : i32 = line.split_whitespace().last().unwrap().parse().unwrap();
            x += number;
            // Part2: update position_sprite
            position_sprite = position_sprite + number;
        }
        
        // increase cycle_num
        cycle_num+=1;

        // check if cycle_num is in array for summing in part1
        if array.contains(&cycle_num) {
            sum += cycle_num*x
        }
    }
    println!("Part1: sum: {}", sum);
    // print out the CRT
    println!("Part2:");

    for line in crt.iter() {
        for character in line.iter() {
            print!("{}", character);
        }
        println!("");
    }
}
