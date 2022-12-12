


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
    let mut previous_location = start;
    
    let mut steps = 0;
    // create an vector of ones of size 4 of (i8, i8)
    let mut possible_moves: Vec<(i8, i8)> = vec![(1, 0),(0, 1),(-1, 0),(0, -1)];
    while current_location != end {
        let mut current_char = map[current_location.0][current_location.1];
        let mut current_char_int = find_index_char_in_alphabet(current_char);
        let mut possible_positions: Vec<(usize, usize)> = Vec::new();

        // get list of possible moves, up down left right
        for possible_move in &mut possible_moves {
            let mut possible_next_position = (current_location.0+ possible_move.0 as usize, current_location.1 + possible_move.1 as usize);
            
            // check if next position is still in the map
            if possible_next_position.0 >= map.len() || possible_next_position.1 >= map[0].len() {
                continue;
            }
            println!(" possible_next_position:  {:?} previous_location {:?}", possible_next_position, previous_location);
            if possible_next_position.0 == previous_location.0 && possible_next_position.1 == previous_location.1 {
                println!("previous location");
                continue;
            }
            let mut possible_next_char = map[possible_next_position.0][possible_next_position.1];
            let mut possible_next_char_int = find_index_char_in_alphabet(possible_next_char);
            if possible_next_char_int <= current_char_int + 1  {
                possible_positions.push(possible_next_position);
            }
        }

        // From the possible_positions, prefer the one with the highest char value
        let mut closest_position = (0, 0);
        let mut closest_char = ' ';
        let mut closest_char_int = 0;
        for possible_position in &possible_positions {
            let mut possible_char = map[possible_position.0][possible_position.1];
            let mut possible_char_int = find_index_char_in_alphabet(possible_char);
            if possible_char_int > closest_char_int {
                closest_char_int = possible_char_int;
                closest_char = possible_char;
                closest_position = *possible_position;
            }
        }
        previous_location = current_location.clone();
        steps += 1;
        current_location = (closest_position.0 as usize, closest_position.1 as usize);
        println!("current location: {:?}", current_location);
        if current_location.0 == end.0 && current_location.1 == end.1 {
            println!("found end at {} steps", steps);
            break;
        }

    }
        

}
