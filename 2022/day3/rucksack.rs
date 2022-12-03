use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let alfabet_lower_upper = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total_priority = 0;
    let mut total_priority2 = 0;

    let mut line_number = 0;
    let mut vec_three_rucksacks = Vec::from([" ".to_string()," ".to_string()," ".to_string()]);
    if let Ok(lines) = read_lines("./example.txt") {
        for line in lines {
            let content = line.unwrap();
            let length = content.chars().count();
            //println!("{} with length {:?}",content, length);
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
            //println!("Common letter {} with priority {}",common_letter, priority.unwrap());
            total_priority += priority.unwrap()+1;

            // Start part2
            vec_three_rucksacks[line_number%3] = content;
            let mut common_letter2 =char::default();
            let mut common_letter3 =char::default();
            if line_number%3 == 2{
                for letter in vec_three_rucksacks[0].chars(){
                    let outcome2 = vec_three_rucksacks[1].find(letter);
                    match outcome2{
                        Some(x) => common_letter2 = letter,
                        None => {},
                    }
                    let outcome3 = vec_three_rucksacks[2].find(common_letter2);
                    match outcome3{
                        Some(x) => common_letter3 = common_letter2,
                        None => {},
                    }

                }
                let priority2 = alfabet_lower_upper.find(common_letter3);
                total_priority2 += priority2.unwrap()+1;
            }
            line_number+=1;


        }
    }
    println!("total score part 1: {}", total_priority);
    println!("total score part 2: {}", total_priority2);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
