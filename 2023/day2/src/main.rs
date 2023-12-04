fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap();
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
        println!("{:?}", numbers);
        for i in 0..numbers.len() {
            //split up numbers by ,
            let color: Vec<&str> = numbers[i].split(",").collect();

            // if string contains "blue"
            for n in 0..color.len() {
                if color[n].contains("blue") {
                    // split string with "blue
                    let color: Vec<&str> = color[n].split("blue").collect();
                    println!("{:?}", color);
                    // get the first element of the vector and convert it to an integer
                    let blue = color[0].trim().parse::<i32>().unwrap();
                    println!("{:?}", blue);
                    if blue > 12 {
                        possible = false;
                    }
                }
                else if color[n].contains("red") {
                    let color: Vec<&str> = color[n].split("red").collect();
                    println!("{:?}", color);
                    let red = color[0].trim().parse::<i32>().unwrap();
                    println!("{:?}", red);
                    if red > 13 {
                        possible = false;
                    }
                }
                else if color[n].contains("green") {
                    let color: Vec<&str> = color[n].split("green").collect();
                    println!("{:?}", color);
                    let green = color[0].trim().parse::<i32>().unwrap();
                    println!("{:?}", green);
                    if green > 14 {
                        possible = false;
                    }
                } else {
                    println!("Error");
                }
                println!("{:?}", possible);
            }

            let number = numbers[i].to_string();

        }
        if possible {
            total_possible_ids += game_number;
        }
    }
println ! ("{:?}", total_possible_ids);
}
