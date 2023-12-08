use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut lines = content.lines();
    let order = lines.next().unwrap();
    let _ = lines.next();

    let directions: Vec<char> = order.chars().collect(); 
    let mut ways: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0];
        let values: Vec<&str> = parts[1].trim_matches(|p| p == '(' || p == ')')
            .split(", ").collect();

        ways.insert(key, (values[0], values[1]));
    }

    let len = directions.len();
    let mut lengths: Vec<usize> = Vec::new();
    let mut keys: Vec<&str> = ways.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k).collect();

    for key in &mut keys {
        let mut index = 0;
        while !key.ends_with('Z') {
            let tuple = ways.get(key).unwrap();
            *key = if directions[index % len] == 'L' { tuple.0 } else { tuple.1 };
            index += 1;
        }

        lengths.push(index);
    }

    let lcm = lengths.iter().fold(1, |acc, &num| lcm(acc, num));
    println!("LCM: {}", lcm);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
