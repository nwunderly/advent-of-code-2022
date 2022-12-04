use std::fs;
use std::collections::HashMap;

fn main() {
    /********\
    | part 1 |
    \********/
    let file = fs::read_to_string("input.txt").expect("Unable to read file.");
    let lines = file.lines();

    let mut p1: &str;
    let mut p2: &str;
    let mut split: Vec<&str>;
    let mut points: i32 = 0;

    for line in lines.clone() {
        split = line.split_whitespace().take(2).collect::<Vec<&str>>();
        [p1, p2] = <[&str; 2]>::try_from(split).ok().unwrap();

        points += points_part_1(p1, p2);
    }

    println!("part 1 total score: {}", points);

    /********\
    | part 2 |
    \********/
    points = 0;
    
    for line in lines.clone() {
        split = line.split_whitespace().take(2).collect::<Vec<&str>>();
        [p1, p2] = <[&str; 2]>::try_from(split).ok().unwrap();

        points += points_part_2(p1, p2);
    }

    println!("part 2 total score: {}", points);
}

fn points_part_1(p1: &str, p2: &str) -> i32 {
    // points for chosen move
    let points = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut score = points[p2];

    // tie
    if [p1, p2] == ["A", "X"] || [p1, p2] == ["B", "Y"] || [p1, p2] == ["C", "Z"] {
        score += 3;
    }
    // win
    else if [p1, p2] == ["A", "Y"] || [p1, p2] == ["B", "Z"] || [p1, p2] == ["C", "X"] {
        score += 6;
    }

    return score
}

fn points_part_2(p1: &str, p2: &str) -> i32 {
    // points for win/loss
    let points = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    let mut score = points[p2];

    // rock
    if [p1, p2] == ["A", "Y"] || [p1, p2] == ["B", "X"] || [p1, p2] == ["C", "Z"] {
        score += 1;
    }
    // paper
    else if [p1, p2] == ["A", "Z"] || [p1, p2] == ["B", "Y"] || [p1, p2] == ["C", "X"] {
        score += 2;
    }
    // scissors
    else {
        score += 3;
    }

    return score
}