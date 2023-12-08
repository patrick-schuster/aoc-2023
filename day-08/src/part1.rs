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
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let mut values = parts.next().unwrap()
            .trim_matches(|p| p == '(' || p == ')')
            .split(", ");

        ways.insert(key, (values.next().unwrap(), values.next().unwrap()));
    }

    let len = directions.len();
    let mut key = "AAA";
    let mut index = 0;
    while key != "ZZZ" {
        let direction = directions[index % len];
        let tuple = ways.get(key).unwrap();
        key = if direction == 'L' { tuple.0 } else { tuple.1 };
        index += 1;
    }

    println!("Steps: {}", index);
}
