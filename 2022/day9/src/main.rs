use std::io::{BufRead, BufReader};
use std::fs::{File};

fn print_a_matrix_and_flip_vertically(matrix: &Vec<Vec<char>>) {
    for i in (0..matrix.len()).rev() {
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j]);
        }
        println!();
    }
    println!();
}

fn remove_previous_head_position(matrix_head: &mut Vec<Vec<char>>) {
    for i in 0..matrix_head.len() {
        for j in 0..matrix_head[i].len() {
            if matrix_head[i][j] == 'H' {
                matrix_head[i][j] = '.';
            }
        }
    }
}

fn remove_previous_head_position_and_get_previous_position(matrix_head: &mut Vec<Vec<char>>) -> (usize, usize) {
    let mut previous_position: (usize, usize) = (0, 0);
    for i in 0..matrix_head.len() {
        for j in 0..matrix_head[i].len() {
            if matrix_head[i][j] == 'H' {
                previous_position = (i , j);
                matrix_head[i][j] = '.';
            }
        }
    }
    previous_position
}

fn remove_previous_tail_position_and_get_previous_position(matrix_tail: &mut Vec<Vec<char>>) -> (usize, usize) {
    let mut previous_position: (usize, usize) = (0, 0);
    for i in 0..matrix_tail.len() {
        for j in 0..matrix_tail[i].len() {
            if matrix_tail[i][j] == 'T' {
                previous_position = (i , j);
                matrix_tail[i][j] = '.';
            }
        }
    }
    previous_position
}

fn remove_previous_tail_position(matrix_tail: &mut Vec<Vec<char>>) {
    for i in 0..matrix_tail.len() {
        for j in 0..matrix_tail[i].len() {
            if matrix_tail[i][j] == 'T' {
                matrix_tail[i][j] = '.';
            }
        }
    }
}

fn tail_postion_based_on_head_postion(x_head: usize, y_head: usize, x_tail: usize, y_tail: usize) -> (i32, i32) {
    let mut x_tail_new = 0;
    let mut y_tail_new = 0;
    //println!("x_head: {}, y_head: {}, x_tail: {}, y_tail: {}", x_head, y_head, x_tail, y_tail);

    // check if tail position is at more than 1 step away from head position to determine if the tail position needs to be updated
    if x_head as i32  >= x_tail as i32 - 1 && x_head as i32 <= x_tail as i32 + 1  && y_head as i32 >= y_tail as i32 - 1 && y_head as i32<= y_tail as i32 + 1 {
        // tail position is at most 1 step away from head position
        // do nothing
    } else {
        // change the tail position with one step closer to head position in x y and diagonal directions
        if x_head > x_tail {
            x_tail_new =  1;
        } else if x_head < x_tail {
            x_tail_new = - 1;
        }
        if y_head > y_tail {
            y_tail_new =  1;
        } else if y_head < y_tail {
            y_tail_new = - 1;
        }
    } 



    (x_tail_new, y_tail_new)
}

fn update_head_position(matrix_head: &mut Vec<Vec<char>>, matrix_tail: &mut Vec<Vec<char>>, matrix_tail_visits: &mut Vec<Vec<char>>, command: &str, steps: usize){

    println!("command: {}, steps: {}", command, steps);
    let mut ix : i32 = 0;
    let mut iy : i32 = 0;
    match command {
        "R" => {
            ix = 0;
            iy = 1;
        },
        "L" => {
            ix = 0;
            iy = -1;
        },
        "U" => {
            ix = 1;
            iy = 0;
        },
        "D" => {
            ix = -1;
            iy = 0;
        },
        _ => panic!("Invalid direction"),        

    }
    let (x,y) = remove_previous_head_position_and_get_previous_position(matrix_head);


    let (mut xt, mut yt) = remove_previous_tail_position_and_get_previous_position(matrix_tail);
    for i in 1..steps+1 {
        remove_previous_head_position(matrix_head);
        let x_pos = (x as i32 + (ix * (i as i32))) as usize;
        let y_pos = (y as i32 + (iy * (i as i32))) as usize;
        matrix_head[x_pos][y_pos] = 'H';
        remove_previous_tail_position(matrix_tail);
        let (xt_new, yt_new) = tail_postion_based_on_head_postion(x_pos, y_pos, xt, yt);
        let x_pos_t = (xt as i32 + (xt_new )) as usize;
        let y_pos_t = (yt as i32 + (yt_new )) as usize;
        matrix_tail[x_pos_t][y_pos_t] = 'T';
        matrix_tail_visits[x_pos_t][y_pos_t] = '#';
        xt = x_pos_t;
        yt = y_pos_t;

        //merge_print_matrix(matrix_head, matrix_tail);
    }
}



fn update_head_and_tail_positions(matrix_head: &mut Vec<Vec<char>>, matrix_tail: &mut Vec<Vec<char>>,matrix_tail_visits: &mut Vec<Vec<char>>, command: &str, steps: usize) {

    // update head position
    update_head_position(matrix_head, matrix_tail, matrix_tail_visits, command, steps);

}

// merge the head and tail matrices into the a single matrix and print it
fn merge_print_matrix(matrix_head: &mut Vec<Vec<char>>, matrix_tail: &mut Vec<Vec<char>>) {

    let mut main_matrix: Vec<Vec<char>> = vec![vec!['.'; matrix_head[0].len()]; matrix_head.len()];
    main_matrix[0][0] = 's';

    // merge the head and tail matrices into the main matrix
    for i in 0..matrix_head.len() {
        for j in 0..matrix_head[i].len() {
            if matrix_head[i][j] == 'H' {
                main_matrix[i][j] = 'H';
            }
            if matrix_tail[i][j] == 'T' {
                main_matrix[i][j] = 'T';
            }
            if matrix_head[i][j] == 'H' && matrix_tail[i][j] == 'T' {
                main_matrix[i][j] = 'H';
            }
        }
    }
    print_a_matrix_and_flip_vertically(&main_matrix);

}

fn count_amount_of_certain_characters_in_matrix(matrix: &mut Vec<Vec<char>>, character: char) -> usize {
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == character {
                count += 1;
            }
        }
    }
    count
}
 

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let mut file_str = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file_str).lines();

    //define a matrix of 255x255 indices of '.'
    let size = 1000;
    //initialize a reference of a matrix
    let mut matrix_head: Vec<Vec<char>> = vec![vec!['.'; size]; size];
    let mut matrix_tail: Vec<Vec<char>> = vec![vec!['.'; size]; size];
    let mut matrix_tail_visits: Vec<Vec<char>> = vec![vec!['.'; size]; size];
    matrix_tail_visits[size/2][size/2] = '#';
    matrix_head[size/2][size/2] = 'H';
    matrix_tail[size/2][size/2] = 'T';

    let mut line_number = 0;
    for line in reader {
        let line_str = line.unwrap();
        let vec_command: Vec<&str> = line_str.split_whitespace().collect();
        //get reference string of vector and parse it to a character
        let mut command = vec_command[0];
        let steps = vec_command[1].parse::<usize>().unwrap();

        update_head_and_tail_positions(&mut matrix_head, &mut matrix_tail, &mut matrix_tail_visits, command, steps);
        //merge_print_matrix(&mut matrix_head, &mut matrix_tail);

        println!("{}", line_number);
        line_number += 1;

    }

    print_a_matrix_and_flip_vertically(&matrix_tail_visits);
    println!("Amount of tail visits: {}", count_amount_of_certain_characters_in_matrix(&mut matrix_tail_visits, '#'));
}