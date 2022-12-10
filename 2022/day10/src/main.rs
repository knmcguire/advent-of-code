use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    // Read in a input file, of which the name is passed in as a command line argument
    let args: Vec<String> = std::env::args().collect();
    let file_str = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file_str).lines();

    // Loop through the lines of the file and retrieve the string
    let mut vec_str : Vec<String> = Vec :: new();
    let mut vec2_str : Vec<String> = Vec :: new();

    for line in reader {
        let line_str = line.unwrap();
        if line_str.contains("noop") {
            vec_str.push("noop".to_string());
            vec2_str.push("noop".to_string());
        } else {
            let instruction = line_str.clone();
            vec_str.push("noop".to_string());
            vec2_str.push(instruction.clone());
            vec_str.push(instruction.clone());
            vec2_str.push("noop".to_string());
        }
    }
    let mut x : i32 = 1;
    let mut cycle_num = 1;
    let mut sum = 0;
    println!("vec2_str.len: {:?}", vec2_str.len());
    println!("vec_str.len: {:?}", vec_str.len());


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
            //println!("cycle_num: {}, x: {}, {}", cycle_num, x, cycle_num*x);
            sum += cycle_num*x
        }
        //println!("cycle_num: {}, x: {}", cycle_num, x);
    }
    println!("sum: {}", sum);

    // make a string that is 40 characters long with '.' as the default character
    let mut sprite = String::with_capacity(40);
    sprite.push_str(&"........................................");
    // replace the first 3 characters with '#'
    sprite.replace_range(..3, "###");
    // Make a vector of 6 instances of empty strings  that contains 40 spots
    let mut CRT : Vec<Vec<char>> = vec![vec![' '; 40]; 6];

    cycle_num = 1;
    let mut crt_level: usize = 0;
    for line in vec2_str.iter() {
        if line.contains("noop") {
            let current_index = ((cycle_num-1) % 40 ) as usize;
            crt_level = ((cycle_num-1)  / 40 )as usize  ;
            let current_drawn_character = sprite.chars().nth(current_index).unwrap();
            CRT[crt_level][current_index] = current_drawn_character;
            cycle_num += 1;
        } else {
            

            // second cycle of addx
            let current_index = ((cycle_num-1) % 40 ) as usize;
            crt_level = ((cycle_num-1) / 40 ) as usize;
            let current_drawn_character = sprite.chars().nth(current_index).unwrap();
            CRT[crt_level][current_index] = current_drawn_character;

            let number : i32 = line.split_whitespace().last().unwrap().parse().unwrap();
            // In sprite, find the location of the "###" and give position
            let position = sprite.find("###").unwrap() + 1;
            println!("position: {} x {}", position, number);
            // remove the "###" from the sprite
            sprite.replace_range(position-1..position+2, "...");
            // add the "###" to the new position
            if position as i32 + number < 0 {
                sprite.replace_range(0..3, "###");
            } else if position as i32 + number > 39 {
                sprite.replace_range(37..40, "###");
            } else {
                sprite.replace_range((position as i32 + number - 1) as usize..(position as i32 + number + 2) as usize, "###");
            }
            cycle_num+= 1;

        }
        //println!("cycle_num: {}, x: {}", cycle_num, x);
    }
    // print out the CRT
    for line in CRT.iter() {
        for character in line.iter() {
            print!("{}", character);
        }
        println!("");
    }

}
