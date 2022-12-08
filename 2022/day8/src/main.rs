use std::fs::{File, remove_dir_all, remove_file, create_dir};
use std::io::{BufRead, BufReader, Write};

fn get_vector_numbers_in_direction(matrix: &Vec<Vec<i32>>, i: usize, j: usize, direction: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    match direction {
        "right" => {
            for k in j+1..matrix[i].len() {
                numbers.push(matrix[i][k]);
            }
        },
        "left" => {
            for k in (0..j).rev() {
                numbers.push(matrix[i][k]);
            }
        },
        "up" => {
            for k in (0..i).rev() {
                numbers.push(matrix[k][j]);
            }
        },
        "down" => {
            for k in i+1..matrix.len() {
                numbers.push(matrix[k][j]);
            }
        },
        _ => panic!("Invalid direction"),
    }
    numbers
}

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let mut file_str = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file_str).lines();

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in reader {
        let line_str = line.unwrap();
        let mut row: Vec<i32> = Vec::new(); 
        // get each character in string, parse it to i32 and push it to row
        for c in line_str.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        println!("{:?}", row);

        matrix.push(row);
    }

    // Count how many indexes are on the outsides of the matrix
    let mut count1 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == 0 || i == matrix.len()-1 || j == 0 || j == matrix[i].len()-1 {
                count1 += 1;
            }
        }
    }
    println!{"{:?}, {:?}", matrix.len(), matrix[0].len()};

    let mut count2 = 0;

    // Got through all the indices of the matrix, avoiding the rows and columns
    // that are on the outsides of the matrix
    for i in 1..matrix.len()-1 {
        for j in 1..matrix[i].len()-1 {
            let mut is_max = true;
            let right = get_vector_numbers_in_direction(&matrix, i, j, "right");
            let left = get_vector_numbers_in_direction(&matrix, i, j, "left");
            let up = get_vector_numbers_in_direction(&matrix, i, j, "up");
            let down = get_vector_numbers_in_direction(&matrix, i, j, "down");

            // Compare the highest number of a vector with the current number

            let right_max = right.iter().max().unwrap();
            let left_max = left.iter().max().unwrap();
            let up_max = up.iter().max().unwrap();
            let down_max = down.iter().max().unwrap();

            if matrix[i][j] > *right_max || matrix[i][j] > *left_max || matrix[i][j] > *up_max || matrix[i][j] > *down_max {
                count2 += 1;
            }


        }
    }

    println!("count1: {} count2 {} total {}", count1, count2, count1 + count2);
}
