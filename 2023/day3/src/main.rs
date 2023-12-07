use std::io::{stdout, Write};


fn print_grid(grid: &Vec<Vec<char>>) -> std::io::Result<()> {
    // Clear the screen
    //print!("\x1B[2J\x1B[1;1H");

    // Print the grid
    for row in grid {
        for &ch in row {
            print!("{} ", ch);
        }
        println!();
    }
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Flush the output to ensure it gets displayed
    stdout().flush()

}

fn main()  {
    // Get string from input text
    let input = std::fs::read_to_string("example1.txt").unwrap();


    // put each character in a 2d vector
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);

    }

    //print_grid(&grid);

    // Go through the grid character for character
    // If the character is a number, collect the next 2 characters
    let mut valid_numbers: Vec<i32> = Vec::new();
    for i in 0..grid.len() {
        let mut j = 0;
        while j < grid[i].len() {
            let mut valid_number = false;
            if grid[i][j].is_numeric() {
                let mut number = String::new();
                let mut k = j;
                while k < grid[j].len() && grid[i][k].is_numeric() {
                    // Get the row and column items around the numberf
                    number.push(grid[i][k]);
                    for l in i.saturating_sub(1)..=i+1 {
                        for m in k.saturating_sub(1)..=k+1 {
                            if l < grid.len() && m < grid[i].len() {
                                // if character is not numeric and not a '.', print something
                                // if there is nothing around the number that is not a number or a '.', it is a invalid number
                                if grid[l][m].is_numeric() || grid[l][m] == '.' {
                                } else {
                                    valid_number = true;
                                }
                            }
                        }
                    }
                    k += 1;
                }
                if valid_number {
                    valid_numbers.push(number.parse::<i32>().unwrap());
                }
                j = k;
            } else {
                j += 1;
            }
        }
    }

    // sum of valid numbers is
    let mut sum = 0;
    for number in valid_numbers {
        sum += number;
    }
    println!("Day3 Part 1 -- Total sum of valid part numbers: {}", sum);

}
