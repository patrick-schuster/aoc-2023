use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let lines = content.lines();
    let mut rounds: Vec<(String, u16)> = Vec::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let hand = split.next().unwrap()
            .chars().map(map_characters)
            .collect::<String>();

        let bid: u16 = split.next().unwrap()
            .parse().unwrap();
    
        rounds.push((hand, bid));
    }

    // Sort by value and lexographically.
    rounds.sort_by(|a, b| 
        (evaluate(a), &a.0).cmp(&(evaluate(b), &b.0)));

    // Calculate the sum.
    let mut sum: u32 = 0;
    for i in 0..rounds.len() {
        let round = &rounds[i];
        sum += round.1 as u32 * (i as u32 + 1);
    }

    println!("Sum: {}", sum);
}

fn map_characters(c: char) -> char {
    match c {
        'A' => 'M',
        'K' => 'L',
        'Q' => 'K',
        'J' => 'J',
        'T' => 'I',
        '9' => 'H',
        '8' => 'G',
        '7' => 'F',
        '6' => 'E',
        '5' => 'D',
        '4' => 'C',
        '3' => 'B',
        '2' => 'A',
        t => t
    }
}

fn evaluate(round: &(String, u16)) -> u8 {
    let mut map: HashMap <char, u8> = HashMap::new();
    for c in round.0.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    // Logic for calculating the value.
    let len = map.len();
    match len {
        1 => 7,
        2 => {
            let first = map.values().next().unwrap();
            return if *first == 1 || *first == 4 { 6 } else { 5 };
        }
        3 => if map.values().any(|&x| x == 3) { 4 } else { 3 },
        4 => 2,
        5 => 1,
        _ => 0
    }
}