use std::fs;


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

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Node {
    position: (usize, usize),
    g: u32,
    h: u32,
    f: u32,
    parent: (usize, usize),
}

fn are_nodes_equal(node1: &Node, node2: &Node) -> bool {
    node1.position.0 == node2.position.0 && node1.parent.1 == node2.parent.1
    && node1.position.1 == node2.position.1 && node1.parent.0 == node2.parent.0
    && node1.g == node2.g && node1.h == node2.h && node1.f == node2.f
}

fn path_searching(map: Vec<Vec<char>>, start: (usize,usize), end: (usize,usize)) -> usize {

    // Create a tuple of None for usize
    let mut steps = 0;
    let start_node = Node {
        parent: (usize::MAX, usize::MAX),
        position: start,
        g: 0,
        h: 0,
        f: 0,
    };
    let end_node = Node {
        parent: (usize::MAX, usize::MAX),
        position: end,
        g: 0,
        h: 0,
        f: 0,
    };

    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    open_list.push(start_node);

    //for it in 0..10 {
    //    println!("it: {}", it);
    while open_list.len()>0 {
        println!("open_list: {:?}", open_list.len());
        // get current node
        let mut current_node = open_list[0].clone();
        let mut current_index = 0;
        for (i, node) in open_list.iter().enumerate() {
            if node.f < current_node.f {
                current_node = node.clone();
                current_index = i;
            }
        }
        println!("current_node: {:?}", current_node);
        // remove current node from open list
        open_list.remove(current_index);
        closed_list.push(current_node.clone());

        // check if current node is the end node
        if current_node.position == end_node.position && current_node.parent == end_node.parent {
            let mut temp = current_node;
            while temp.parent != (usize::MAX, usize::MAX) {
                steps += 1;
                temp = closed_list.iter().find(|&x| x.position == temp.parent).unwrap().clone();
            }
            return steps;
        }

        // get children of current node
        let mut children: Vec<Node> = Vec::new();
        let mut possible_moves: Vec<(i8, i8)> = vec![(1, 0),(0, 1),(-1, 0),(0, -1)];

        for new_move in possible_moves {
            let mut new_position = ((current_node.position.0 as i32 + new_move.0 as i32) as usize, (current_node.position.1 as i32 + new_move.1 as i32)as usize);

            // check if new position is still in the map
            if new_position.0 >= map.len() || new_position.1 >= map[0].len() {
                continue;
            }
            // check the character and interger in new positition
            let mut new_char = map[new_position.0][new_position.1];
            let mut new_char_int = find_index_char_in_alphabet(new_char);
            let mut current_char = map[current_node.position.0][current_node.position.1];
            let mut current_char_int = find_index_char_in_alphabet(current_char);
            if new_char_int > current_char_int + 1 {
                continue;

            }

            let mut new_node = Node {
                parent: current_node.position,
                position: (new_position.0 as usize, new_position.1 as usize),
                g: 0,
                h: 0,
                f: 0,
            };

            children.push(new_node);
            // go through all children
            for child in &mut children {
                // check if child is in closed list
                if closed_list.contains(child) {
                    println!("Already in closed list");
                    continue;
                }

                println!("next child");

                child.g = current_node.g + 1;
                child.h = ((child.position.0 as i32 - end_node.position.0 as i32).abs() + (child.position.1 as i32 - end_node.position.1 as i32).abs()) as u32;
                child.f = child.g + child.h;

                println!("child: {:?}", child);
                for open_node in &mut open_list {
                    //println!("open_node: {:?}", open_node);
                    //println!("child: {:?}", child);
                    if child.position == open_node.position && child.parent == open_node.parent && child.g > open_node.g {
                        continue;
                    }
                }
                println!("push{:?}", child);
                open_list.push(child.clone());
            }
        }
        

    }

    steps
}

fn path_searching_v1(map: Vec<Vec<char>>, start: (usize,usize), end: (usize,usize)) -> usize
{
    let mut current_location = start;
    let mut previous_location = start;
    let mut previous_locations = Vec::new();
    previous_locations.push(start);
    
    let mut steps = 0;
    // create an vector of ones of size 4 of (i8, i8)
    let mut possible_moves: Vec<(i8, i8)> = vec![(1, 0),(0, 1),(-1, 0),(0, -1)];
    while current_location != end {
        let mut current_char = map[current_location.0][current_location.1];
        let mut current_char_int = find_index_char_in_alphabet(current_char);
        let mut possible_positions: Vec<(usize, usize)> = Vec::new();

        // get list of possible moves, up down left right
        for possible_move in &mut possible_moves {
            println!("current location: {:?} {:?} ", current_location, possible_move);
            let mut possible_next_position = ((current_location.0 as i32 + possible_move.0 as i32) as usize, (current_location.1 as i32 + possible_move.1 as i32) as usize);
            
            // check if next position is still in the map
            if possible_next_position.0 >= map.len() || possible_next_position.1 >= map[0].len() {
                println!("out of map");
                continue;
            }
            if previous_locations.contains(&possible_next_position) {
                println!("previous location");
                continue;
            }
            let mut possible_next_char = map[possible_next_position.0][possible_next_position.1];
            let mut possible_next_char_int = find_index_char_in_alphabet(possible_next_char);
            if possible_next_char_int <= current_char_int + 1  {
                possible_positions.push(possible_next_position);
            }
        }

        println!("possible positions: {:?}", possible_positions);
        // From the possible_positions, store the distances to the end position
        let mut distances: Vec<usize> = Vec::new();
        for possible_position in &possible_positions {
            let mut distance = (possible_position.0 as i32 - end.0 as i32).abs() + (possible_position.1 as i32 - end.1 as i32).abs();
            distances.push(distance as usize);
        }

        // provide a list of indexes of the smallest distances
        let mut smallest_distance = distances[0];
        let mut smallest_distance_indexes: Vec<usize> = Vec::new();
        for (i, distance) in distances.iter().enumerate() {
            if *distance < smallest_distance {
                smallest_distance = *distance;
                smallest_distance_indexes.clear();
                smallest_distance_indexes.push(i);
            } else if *distance == smallest_distance {
                smallest_distance_indexes.push(i);
            }
        }

        // From the possible_positions, prefer the one with the highest char value
        let mut closest_position = (0, 0);
        let mut closest_char = ' ';
        let mut closest_char_int = 0;
        for n in &smallest_distance_indexes {
            let possible_position = possible_positions[*n];
            let mut possible_char = map[possible_position.0][possible_position.1];
            let mut possible_char_int = find_index_char_in_alphabet(possible_char);
            if possible_char_int > closest_char_int {
                closest_char_int = possible_char_int;
                closest_char = possible_char;
                closest_position = possible_position;
            }
        }
        println!("distance: {:?}", distances);
        println!("possible positions: {:?}", possible_positions);


        previous_location = current_location.clone();
        previous_locations.push(current_location.clone());
        steps += 1;
        current_location = (closest_position.0 as usize, closest_position.1 as usize);
        println!("current location: {:?}", current_location);
        if current_location.0 == end.0 && current_location.1 == end.1 {
            println!("found end at {} steps", steps);
            break;
        }

    }
    steps
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

    path_searching_v1(map, start, end);        

}
