use std::collections::HashMap;
use std::io;
use std::fs;
use std::path;
use std::io::BufRead;

fn main() {

    let mut rps_scores = HashMap::new();
    rps_scores.insert("X".to_string(), 1);
    rps_scores.insert("Y".to_string(), 2);
    rps_scores.insert("Z".to_string(), 3);

    let mut rps_scores2 = HashMap::new();
    rps_scores2.insert("A".to_string() , 1 as i32);
    rps_scores2.insert("B".to_string()  , 2 as i32);
    rps_scores2.insert("C".to_string()  , 3 as i32);

    let mut outcome_scores = HashMap::new();
    outcome_scores.insert("Lose".to_string(), 0);
    outcome_scores.insert("Draw".to_string(), 3);
    outcome_scores.insert("Win".to_string(), 6);

    let mut outcome_scores2 = HashMap::new();
    outcome_scores2.insert("X".to_string(), 0);
    outcome_scores2.insert("Y".to_string(), 3);
    outcome_scores2.insert("Z".to_string(), 6);

    let combo_win = HashMap::from([
        ("A Y".to_string(), "Win".to_string()),
        ("B Z".to_string(), "Win".to_string()),
        ("C X".to_string(), "Win".to_string()),
        ("B X".to_string(), "Lose".to_string()),
        ("C Y".to_string(), "Lose".to_string()),
        ("A Z".to_string(), "Lose".to_string()),
        ("A X".to_string(), "Draw".to_string()),
        ("B Y".to_string(), "Draw".to_string()),
        ("C Z".to_string(), "Draw".to_string()),
    ]);

    let combo_win2 = HashMap::from([
        ("A Y".to_string(), "A".to_string()),
        ("B Z".to_string(), "C".to_string()),
        ("C X".to_string(), "B".to_string()),
        ("B X".to_string(), "A".to_string()),
        ("C Y".to_string(), "C".to_string()),
        ("A Z".to_string(), "B".to_string()),
        ("A X".to_string(), "C".to_string()),
        ("B Y".to_string(), "B".to_string()),
        ("C Z".to_string(), "A".to_string()),
    ]);
    
    let filename = "input.txt".to_string();
    let mut total_score = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let result = line.unwrap();
            let split = result.split(" ");
            let vec = split.collect::<Vec<&str>>();
            
            let mut current_score = 0;
            /*if let Some( outcome) = combo_win.get(&result){
                if let Some(tmp_score) = outcome_scores.get(&*outcome)
                {
                    current_score +=tmp_score;
                }
            }*/
            if let Some(outcome) = combo_win2.get(&result){
                println!("{:?}",outcome);
                if let Some(tmp_score) = rps_scores2.get(&*outcome)
                {
                    println!("{:?}",tmp_score);
                    current_score +=tmp_score;
                }
            }
            /*if let Some(tmp_score2) = rps_scores.get(vec[1])
            {
                println!("{:?}",vec[1]);
                println!("{:?}",tmp_score2);
                current_score+=tmp_score2;
            }*/
            if let Some(tmp_score2) = outcome_scores2.get(vec[1])
            {
                println!("{:?}",vec[1]);
                println!("{:?}",tmp_score2);
                current_score+=tmp_score2;
            }
            total_score +=current_score;
            current_score=0;
        }
    }
    println!("{:?}",total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<path::Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

