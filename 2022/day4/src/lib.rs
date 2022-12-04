use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {

    let mut fully_contained_set_count:u32 = 0;
    for line in input.lines()
    {
        let mut split = line.trim().split(','); 
        let mut vec = Vec::new();
        vec = split.collect();
        
        println!("{:?}",vec);
        let mut range1_vec: Vec<&str> = Vec::from(vec[0].split('-').collect());

        //let mut range1_vec: Vec<str> =  vec[0].split('-').collect()[0];
        let range1_left = range1_vec[0].to_digit(10).unwrap();
        let range1_right = vec[1].to_digit(10).unwrap();
        let range1: Vec<u32> = (range1_left..range1_right).map(u32::from).collect();
        let range1_set: HashSet<u32> = HashSet::from_iter(range1);
        let range2_left = vec[1].chars().nth(0).unwrap().to_digit(10).unwrap();
        let range2_right = vec[1].chars().nth(2).unwrap().to_digit(10).unwrap();
        let range2: Vec<u32> = (range2_left..range2_right).map(u32::from).collect();
        let range2_set: HashSet<u32> = HashSet::from_iter(range2);


        if range1_set.is_subset(&range2_set)
        { 
            fully_contained_set_count+=1;
        }
        if range2_set.is_subset(&range1_set)
        { 
            fully_contained_set_count+=1;
        }
        dbg!(fully_contained_set_count);
    }

    fully_contained_set_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input;
    result.to_string()
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