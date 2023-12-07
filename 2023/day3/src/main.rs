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
    println!();
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Flush the output to ensure it gets displayed
    stdout().flush()

}

fn input_to_vector(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);

    }
    grid
}




fn collect_numbers_if_connected_to_symbol(grid: &Vec<Vec<char>>, look_for_one: bool , symbol: char) -> Vec<i32> {

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
                                    if look_for_one {
                                        if grid[l][m] == symbol {
                                            valid_number = true;
                                        }
                                    } else {
                                        valid_number = true;
                                    }
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
    valid_numbers
}

// go through grid, and find number sequences like in part1. For each number sequence, check if it is connected to a symbol
// if it is, add it to a vector


fn part2(input: &str) {
    let grid = input_to_vector(input);
    //find coordinates of symbol '*'
    let mut symbol_coordinates: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' {
                symbol_coordinates.push((i, j));
            }
        }
    }

    // get the indices of the 'symbol' in the grid in a 2d vector
    let mut symbol_indices: Vec<Vec<usize>> = Vec::new();
    for i in 0..grid.len() {
        let mut row: Vec<usize> = Vec::new();
        let mut found_symbol = false;
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' {
                row.push(i);
                row.push(j);
                found_symbol = true;
            }
        }
        if found_symbol {
            symbol_indices.push(row);
        }
    }


    let mut valid_numbers_and_symbol: Vec<Vec<i32>> = Vec::new();
    let look_for_one = true;
    for i in 0..grid.len() {
        let mut j = 0;
        while j < grid[i].len() {
            let mut valid_number = false;
            let mut symbol_index = 0;
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
                                    if look_for_one {
                                        if grid[l][m] == '*' {
                                            valid_number = true;
                                            // check if l and m are in symbol_indices
                                            for n in 0..symbol_indices.len() {
                                                if symbol_indices[n][0] == l && symbol_indices[n][1] == m {
                                                    symbol_index = n;
                                                }
                                            }
                                        }
                                    } else {
                                        valid_number = true;
                                    }
                                }
                            }
                        }
                    }
                    k += 1;
                }
                if valid_number {
                    // add valid numbers and symbol index to vector valid_numbers_and_symbol
                    let mut valid_number_and_symbol: Vec<i32> = Vec::new();
                    valid_number_and_symbol.push(number.parse::<i32>().unwrap());
                    valid_number_and_symbol.push(symbol_index as i32);
                    valid_numbers_and_symbol.push(valid_number_and_symbol);

                }
                j = k;
            } else {
                j += 1;
            }
        }
    }

    // go through valid_number_and_symbol and remove any entry where the symbol id only appears once
    let mut valid_numbers_and_symbol2: Vec<Vec<i32>> = Vec::new();
    for i in 0..valid_numbers_and_symbol.len() {
        let mut symbol_index = valid_numbers_and_symbol[i][1] as usize;
        let mut symbol_count = 0;
        for j in 0..valid_numbers_and_symbol.len() {
            if symbol_index == valid_numbers_and_symbol[j][1] as usize {
                symbol_count += 1;
            }
        }
        if symbol_count > 1 {
            valid_numbers_and_symbol2.push(valid_numbers_and_symbol[i].clone());
        }
    }


    println!("symbol_indices: {:?}", symbol_indices);
    println!("valid_numbers_and_symbol: {:?}", valid_numbers_and_symbol);
    println!("valid_numbers_and_symbol2: {:?}", valid_numbers_and_symbol2);

}



fn part1(input: &str) {
    // put each character in a 2d vector
    let grid = input_to_vector(input);

    let valid_numbers = collect_numbers_if_connected_to_symbol(&grid, false, '*');
    //print_grid(&grid);

    // Go through the grid character for character
    // If the character is a number, collect the next 2 characters


    // sum of valid numbers is
    let mut sum = 0;
    for number in valid_numbers {
        sum += number;
    }
    println!("Day3 Part 1 -- Total sum of valid part numbers: {}", sum);
}

fn main()  {
    // Get string from input text
    let input = std::fs::read_to_string("example1.txt").unwrap();

    part1(&input);
    part2(&input);


}
