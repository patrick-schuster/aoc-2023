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
    let mut key = "AAA";
    let mut index = 0;
    loop {
        let direction = directions[index % len];
        let tuple = ways.get(key).unwrap();
        key = if direction == 'L' { tuple.0 } else { tuple.1 };
        index += 1;

        if key == "ZZZ" {
            println!("Found ZZZ in {} steps", index);
            break;
        }
    }
}
