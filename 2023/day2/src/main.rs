fn part1(input: &str)
{
    // parse input into a 2n vector, rows corresponding to the color
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 4 red, 5 blue; 1 red, 2 green, 6 blue; 2 green
    let mut total_possible_ids = 0;
    let mut game_number = 0;

    for line in input.lines() {
        let mut possible = true;
        game_number += 1;

        // put line in a string
        let numbers = line.to_string();
        // split strings with : or ;
        let numbers: Vec<&str> = numbers.split(|c| c == ':' || c == ';').collect();

        for i in 1..numbers.len() {
            //split up numbers by ,
            let color: Vec<&str> = numbers[i].split(",").collect();

            // if string contains "blue"
            for n in 0..color.len() {
                if color[n].contains("blue") {
                    let color: Vec<&str> = color[n].split("blue").collect();
                    let blue = color[0].trim().parse::<i32>().unwrap();
                    if blue == 14 {
                    }
                    if 14 < blue {
                        possible = false;
                    }
                }
                else if color[n].contains("red") {
                    let color: Vec<&str> = color[n].split("red").collect();
                    let red = color[0].trim().parse::<i32>().unwrap();
                    if 12 < red {
                        possible = false;
                    }
                }
                else if color[n].contains("green") {
                    let color: Vec<&str> = color[n].split("green").collect();
                    let green = color[0].trim().parse::<i32>().unwrap();
                    if 13 < green {
                        possible = false;
                    }
                } else {
                    println!("Error: color not found");
                }
            }
        }


        if possible {
            total_possible_ids += game_number;
        }
    }
    println!("Day2 Part 1 -- Total sum of ids: {}", total_possible_ids);
}

fn part2(input: &str) {
    let mut total_possible_ids = 0;

    for line in input.lines() {
        let mut total_blue = 0;
        let mut total_red = 0;
        let mut total_green = 0;
        // put line in a string
        let numbers = line.to_string();
        // split strings with : or ;
        let numbers: Vec<&str> = numbers.split(|c| c == ':' || c == ';').collect();

        for i in 1..numbers.len() {
            //split up numbers by ,
            let color: Vec<&str> = numbers[i].split(",").collect();

            // if string contains "blue"
            for n in 0..color.len() {
                if color[n].contains("blue") {
                    let color: Vec<&str> = color[n].split("blue").collect();
                    let blue = color[0].trim().parse::<i32>().unwrap();
                    if blue > total_blue {
                        total_blue = blue;
                    }
                }
                else if color[n].contains("red") {
                    let color: Vec<&str> = color[n].split("red").collect();
                    let red = color[0].trim().parse::<i32>().unwrap();
                    if red > total_red {
                        total_red = red;
                    }
                }
                else if color[n].contains("green") {
                    let color: Vec<&str> = color[n].split("green").collect();
                    let green = color[0].trim().parse::<i32>().unwrap();
                    if green > total_green {
                        total_green = green;
                    }
                } else {
                }
            }

        }
            total_possible_ids += total_blue*total_red*total_green;

    }
    println!("Day2 Part 2 -- Total sum of ids: {}", total_possible_ids);

}

fn main() {
    // Input strings from text file
    let input = std::fs::read_to_string("input.txt").unwrap();
    // find out what type input to be input into a function
    part1(&input);
    part2(&input);
}
