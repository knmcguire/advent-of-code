use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;

fn main() {
    let max_length = 1500;
    let mut elf_number = 0;
    let mut elf_array = vec![0; max_length];
    let mut calory_total_per_elf : i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
            let calories = line.unwrap();
            if calories.eq("") {
                elf_array[elf_number] = calory_total_per_elf;
                calory_total_per_elf = 0;
                elf_number+=1;
            }else{
                let calory_per_elf : i32 = calories.parse().unwrap();
                calory_total_per_elf+=calory_per_elf;
            }
        }
        elf_array[elf_number] = calory_total_per_elf;
    }

    let max_value = elf_array.iter().max().unwrap();
    println!("How many calories has the highest loaded Elf have on him/her: {:?}", max_value);

    let mut max_value_total : i32 = 0;
    let nr_of_top_elves = 3;
    let mut heap = elf_array.iter().copied().collect::<BinaryHeap<i32>>();
    for _ in 0..nr_of_top_elves {
        if let Some(v) = heap.pop() {
            max_value_total +=v
        }
    }
    println!("How many calories has the three highest loaded Elf have on them combined: {:?}", max_value_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
