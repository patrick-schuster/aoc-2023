use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut lines = content.lines();
    let order = lines.next().unwrap();
    let _ = lines.next();

    let directions = order.as_bytes();
    let mut ways: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let mut values = parts.next().unwrap()
            .trim_matches(|p| p == '(' || p == ')')
            .split(", ");

        ways.insert(key, (values.next().unwrap(), values.next().unwrap()));
    }

    let len = directions.len();
    let mut lengths: Vec<usize> = Vec::new();
    let mut keys: Vec<&str> = ways.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k).collect();

    for key in &mut keys {
        let mut index = 0;
        while !key.ends_with('Z') {
            let tuple = *ways.get(key).unwrap();
            *key = if directions[index % len] == b'L' { tuple.0 } else { tuple.1 };
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