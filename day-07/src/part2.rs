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
    rounds.sort_by(|a, b| (evaluate(a), &a.0).cmp(&(evaluate(b), &b.0)));

    // Calculate the sum.
    let mut sum: u32 = 0;
    for i in 0..rounds.len() {
        let round = &rounds[i];
        sum += round.1 as u32 * (i as u32 + 1);
    }

    println!("{}", sum);
}

fn map_characters(c: char) -> char {
    match c {
        'A' => 'V',
        'K' => 'U',
        'Q' => 'T',
        'T' => 'S',
        '9' => 'R',
        '8' => 'Q',
        '7' => 'P',
        '6' => 'O',
        '5' => 'N',
        '4' => 'M',
        '3' => 'L',
        '2' => 'K',
        'J' => 'J',
        t => t
    }
}

fn evaluate(round: &(String, u16)) -> u8 {
    let mut map: HashMap <char, u8> = HashMap::new();

    // Put everything except jokers in the map.
    for c in round.0.chars().filter(|&c| c != 'J') {
        *map.entry(c).or_insert(0) += 1;
    }

    // Find pair with most occurences and replace "J" with it.
    let len = map.len();
    if len > 0 {
        let key = *map.iter().max_by_key(|&(_, v)| v).unwrap().0;
        for _ in round.0.chars().filter(|&c| c == 'J') {
            *map.entry(key).or_insert(0) += 1;
        }
    }

    // Logic for calculating the value.
    match len {
        0 | 1 => 7,
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