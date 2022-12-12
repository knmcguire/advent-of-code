


fn find_index_char_in_alphabet(c: char) -> usize {
    let alfabet = "SabcdefghijklmnopqrstuvwxyzE";
    let mut index = 0;
    for (i, ch) in alfabet.chars().enumerate() {
        if ch == c {
            index = i;
        }
    }
    index
}



fn main() {
    // Get a filename from the command line
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    // Read an input and parse it into a 2d vector of chars
    let input = std::fs::read_to_string(filename).unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // make an interger map replacing the chars with corrensponding integers

    // print the map in a readable way
    for row in &map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    
    // Find the location of the starting point 'S' in map
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (i, j);
            }
            if *c == 'E' {
                end = (i, j);
            }
        }
    }

    let mut current_location = start;
    let mut steps = 0;
    for it in 0..50{
        let mut current_char = map[current_location.0][current_location.1];
        let mut current_char_int = find_index_char_in_alphabet(current_char);
        // Get coordinates of the next location either up, down, left or right
        // Which get's the closest to the end
        // Check the next location's char and convert it to an integer
        // If the next location's integer at most one higher than the current location's integer
        // Then move to the next location

        // Check up
        // Check which location is closer to the end
        let mut k : i8 = 0;
        if current_location.0 > end.0 {
            k = -1;
        } else {
            k = 1;
        }
        let mut m : i8 = 0;
        if current_location.1 > end.1 {
            m = -1;
        } else {
            m = 1;
        } 

        let mut vec_direction : Vec<(i8, i8)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]; // up, right, down, left
        let mut vec_direction_score : Vec<(usize)> = vec![0,0,0,0];
        for p in 0..4 {
            println!("p: {}", p);

            let direction = vec_direction[p];
            let mut new_location = (current_location.0 + direction.0 as usize, current_location.1 + direction.1 as usize);
            // if new_location is outside of bounds of the map, then skip it
            if new_location.0 >= map.len() || new_location.1 >= map[0].len() {
                vec_direction_score.push(0);
                continue;
            }
            let mut up_char = map[new_location.0][new_location.1];
            let mut up_char_int = find_index_char_in_alphabet(up_char);
            if up_char_int <= current_char_int + 1 {
                // Check which location is closer to the end
                let mut score = 0;
                if (new_location.0 >= map.len() || new_location.1 >= map[0].len()) && up_char_int <= current_char_int + 1 {

                    if new_location.0 > end.0 {
                        score += new_location.0 - end.0;
                    } else {
                        score += end.0 - new_location.0;
                    }
                    if new_location.1 > end.1 {
                        score += new_location.1 - end.1;
                    } else {
                        score += end.1 - new_location.1;
                    }
                }
                vec_direction_score.push(score);
            } 
        }
        // find index of hightest score
        let mut index = 0;
        let mut highest_score = 0;
        for (i, score) in vec_direction_score.iter().enumerate() {
            if *score > highest_score {
                highest_score = *score;
                index = i;
            }
        }
        println!("index: {}", index);
        // move to the location with the highest score
        current_location = (current_location.0 + vec_direction[index].0 as usize, current_location.1 + vec_direction[index].1 as usize);

        println!("current_location: {:?}", current_location);

        if current_location == end {
            println!("break");
            break;
        }


    }




}
