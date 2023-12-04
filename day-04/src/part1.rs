use std::fs;
use std::collections;

fn main() {
    let input = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    let mut sum: u32 = 0;
    let mut set: collections::HashSet<&str> = collections::HashSet::new();
    let cards = input.split("\n");
    
    for card in cards {
        set.clear();
        let values: Vec<&str> = card.split(" | ").collect();
        let keys: Vec<&str> = values[0].split(": ").collect::<Vec<&str>>()[1].split(" ").collect();
        let values: Vec<&str> = values[1].split(" ").collect();
        
        // Put all non-empty keys in the set.
        for key in keys {
            if !key.is_empty() {
                set.insert(key);
            }
        }
        
        let mut add: u32 = 0;
        for value in values {
            if !value.is_empty() && set.contains(value) {
                add = (add * 2).max(1);
            }
        }

        sum += add;
    }

    println!("Sum: {}", sum)
}