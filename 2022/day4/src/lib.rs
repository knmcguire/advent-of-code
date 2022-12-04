use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {

    let mut fully_contained_set_count:u32 = 0;
    for line in input.lines()
    {

        let (range1_set, range2_set) = get_sets(line);
        
        if range1_set.is_subset(&range2_set)
        { 
            fully_contained_set_count+=1;
        }
        else if range2_set.is_subset(&range1_set)
        { 
            fully_contained_set_count+=1;
        }
    }

    fully_contained_set_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input;
    result.to_string()
}

fn get_sets(input: &str) -> (HashSet<i32>,HashSet<i32>)  {
    let  vec: Vec<&str> = input.trim().split(',').collect(); 

    let range1_vec: Vec<&str> = vec[0].split('-').collect();
    let range1_left = range1_vec[0].parse::<i32>().unwrap();
    let range1_right = range1_vec[1].parse::<i32>().unwrap();
    let range1 = range1_left..range1_right+1;
    let range1_set: HashSet<i32> = range1.collect();

    let range2_vec: Vec<&str> = vec[1].split('-').collect();
    let range2_left = range2_vec[0].parse::<i32>().unwrap();
    let range2_right = range2_vec[1].parse::<i32>().unwrap();
    let range2 = range2_left..range2_right+1;
    let range2_set: HashSet<i32> = range2.collect();

    return (range1_set, range2_set)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[ignore]
    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}