use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn is_unique(v: &VecDeque<char>) -> bool {
    let mut unique = true;
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i != j && v[i] == v[j] {
                unique = false;
            }
        }
    }
    unique
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut sequence = BufReader::new(file).lines().next().unwrap().unwrap().to_string();
    // Seperate sequence string to characters
    let mut sequence_last_four_chars: VecDeque<char> = VecDeque::with_capacity(14);
    let mut it: u32 = 1;
    let distinct_char_number = 14;
    for letter in sequence.chars(){
        sequence_last_four_chars.push_back(letter);

        if it > 14-1{
            if is_unique(&sequence_last_four_chars)
            {
                dbg!("Unique");
                break;
            }
            sequence_last_four_chars.remove(0);
        }

        it += 1;
    }
    println!("Sequence is unique after character  {:?}", it)
}
