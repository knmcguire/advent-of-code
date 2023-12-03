fn part1(input: &str) {
    // initialize total sum
    let mut total_sum: i32 = 0;
    // Go through the string in a forloop for each line
    for line in input.lines() {
        // Get the number from the line by combining the first and last numeric character and convert it to a i32
        let mut numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
        let first = numbers.chars().next().unwrap();
        let last = numbers.chars().last().unwrap();
        numbers = format!("{}{}", first, last);
        let numbers: i32 = numbers.parse().unwrap();
        total_sum += numbers;
    }
    println!("Day1 Part 1 -- Total sum: {}", total_sum);
}

fn part2(input: &str) {
    // initialize total sum
    let mut total_sum: i32 = 0;
    // Go through the string in a forloop for each line
    for line in input.lines() {
        // Get the number from the line by combining the first and last numeric character and convert it to a i32
        let numbers = find_numbers(line);
        // combine the first and last number as 2 characters
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        let numbers = format!("{}{}", first, last);
        let numbers: i32 = numbers.parse().unwrap();
        total_sum += numbers;
    }
    println!("Day1 Part 2 -- Total sum: {}", total_sum);
}

fn find_numbers(input: &str) -> Vec<i32> {
    let numbers = [("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];
    let mut found_numbers = Vec::new();
    let mut start = 0;
    while start < input.len() {
        if let Some(digit) = input[start..].chars().next().and_then(|c| c.to_digit(10)) {
            found_numbers.push(digit as i32);
        } else {
            for (word, number) in &numbers {
                if input[start..].starts_with(word) {
                    found_numbers.push(*number);
                    break;
                }
            }
        }
        start += 1;
    }
    found_numbers
}



fn main() {
    // Input strings from text file
    let input = std::fs::read_to_string("input1.txt").unwrap();
    // find out what type input to be input into a function
    part1(&input);
    part2(&input);
}