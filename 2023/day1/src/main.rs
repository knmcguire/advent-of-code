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
        println!("{}", numbers);
        total_sum += numbers;
    }
    println!("Total sum: {}", total_sum);
}

fn main() {
    // Input strings from text file
    let input = std::fs::read_to_string("input1.txt").unwrap();
    // find out what type input to be input into a function
    part1(&input);
}