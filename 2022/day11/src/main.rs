use std::fs;


// a struct that is meant to be deserialized from a yaml file of the format:
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3 
#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<u32>,
    operation: String,
    test: u32,
    if_true: u32,
    if_false: u32,
    inspection_count: u32,
}


fn main() {
    // Get file name from terminal argument input and read to string
    let filename = std::env::args().nth(1).expect("No file name given");
    let input = std::fs::read_to_string(filename).expect("File not found");
    let mut monkeys = Vec::new();
    // Loop through string, stopping at each line containing "Monkey"
    for line in input.lines() {
        if line.contains("Monkey") {
            // Get the 5 lines after the line containing "Monkey"
            let  monkey_lines = input.lines().skip_while(|&l| l != line).skip(1).take(5);


            // Create a new Monkey struct
            let mut monkey = Monkey {
                starting_items: Vec::new(),
                operation: String::new(),
                test: 0,
                if_true: 0,
                if_false: 0,
                inspection_count: 0,
            };
            // Loop through the 5 lines after the line containing "Monkey"
            for line2 in monkey_lines {
                // If the line contains "Starting items", split the line at the colon and comma
                // and push the numbers to the starting_items vector
                if line2.contains("Starting items") {
                    let split = line2.split(":").skip(1).next().unwrap().split(",");
                    for num in split {
                        monkey.starting_items.push(num.trim().parse().unwrap());
                    }
                }
                if line2.contains("Operation") {
                    monkey.operation = line2.split(":").skip(1).next().unwrap().trim().to_string();
                }
                if line2.contains("Test") {
                    monkey.test = line2.split(": divisible by ").skip(1).next().unwrap().trim().parse().unwrap();
                }
                if line2.contains("If true") {
                    monkey.if_true = line2.split(": throw to monkey").skip(1).next().unwrap().trim().parse().unwrap();
                }
                if line2.contains("If false") {
                    monkey.if_false = line2.split(": throw to monkey").skip(1).next().unwrap().trim().parse().unwrap();
                }
            }
            monkeys.push(monkey);
        }
    }

    for round in 0..20 {
        println!("Round {}", round);
    //print the starting items vector of all monkeys in the vector
    for monkey in &monkeys {
        println!("Monkey {} has {:?} items", monkey.if_true, monkey.starting_items);
    }
        for nr in 0..monkeys.len() {
            // clone the monkey struct
            let  monkey = monkeys[nr].clone();
            let operation = monkey.operation.clone();
            let starting_items = monkey.starting_items.clone();
            let test = monkey.test.clone();
            let if_true = monkey.if_true.clone();
            let if_false = monkey.if_false.clone();

            for nr_item in 0..starting_items.len() {
                // get the current index of the item in the list
                let item = starting_items[nr_item].clone();
                let mut value = item.clone();
                // divide value by three and round of to the nearest integer
                if operation.contains("new = old * old") {
                    value = value * value;
                } else if operation.contains("new = old *") {
                    let multiplier: u32 = operation.split("new = old * ").skip(1).next().unwrap().trim().parse().unwrap();
                    value = value * multiplier;
                }else if operation.contains("new = old +") {
                    let adder: u32 = operation.split("new = old +").skip(1).next().unwrap().trim().parse().unwrap();
                    value = value + adder;
                }
                value = (value as f32 / 3.0) as u32;

                if value % test == 0 {
                    //println!("True: Monkey {} throws {} with value {} to monkey {}", nr, item,value, monkey.if_true);
                    let new_monkey_value = if_true.clone() as usize;
                    let new_monkey = &mut monkeys[new_monkey_value];
                    new_monkey.starting_items.push(value); 
                } else {
                    //println!("False: Monkey {} throws {} with value {} to monkey {}", nr, item,value, monkey.if_false);
                    let new_monkey_value = if_false.clone() as usize;
                    let new_monkey = &mut monkeys[new_monkey_value];
                    new_monkey.starting_items.push(value);
                }
                monkeys[nr].inspection_count += 1;
            }
            // remove all items from the starting_items vector of the original monkey
            monkeys[nr].starting_items.clear();
        }

}
    //print the starting items vector of all monkeys in the vector
    for monkey in &monkeys {
        println!("Monkey {} has {:?} items", monkey.if_true, monkey.starting_items);
    }
    // print all the inspection counts of all monkeys in the vector
    for monkey in &monkeys {
        println!("Monkey {} has inspected {} items", monkey.if_true, monkey.inspection_count);
    }

    // List all the inspection counts in a vector and sort from higest to lowest
    let mut inspection_counts = Vec::new();
    for monkey in &monkeys {
        inspection_counts.push(monkey.inspection_count);
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    println!("Monkey business is is {}", inspection_counts[0]*inspection_counts[1]);
}
