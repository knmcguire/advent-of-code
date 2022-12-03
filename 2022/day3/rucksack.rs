use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let alfabet_lower_upper = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total_priority = 0;
    if let Ok(lines) = read_lines("./example.txt") {
        for line in lines {
            let content = line.unwrap();
            let length = content.chars().count();
            println!("{} with length {:?}",content, length);
            let first_compartment = &content[..length/2];
            let second_compartment = &content[length/2..];
            let mut common_letter = char::default();
            for letter in first_compartment.chars(){
                let outcome = second_compartment.find(letter);
                match outcome{
                    Some(x) => common_letter = letter,
                    None => {},
                }

            }
            let priority = alfabet_lower_upper.find(common_letter);
            println!("Common letter {} with priority {}",common_letter, priority.unwrap());
            total_priority += priority.unwrap();
        }
    }
    println!("total score {}", total_priority);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
