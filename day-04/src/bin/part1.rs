use std::fs;
use std::collections;

fn main() {
    let input = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    let mut set: collections::HashSet<&str> = collections::HashSet::new();
    let cards = input.lines();
    
    for card in cards {
        set.clear();

        let mut iter = card.split(" | ");
        let keys = iter.next().unwrap()
            .split(": ").last().unwrap()
            .split_whitespace();
        let values = iter.next().unwrap()
           .split_whitespace();
        
        // Put all non-empty keys in the set.
        for key in keys {
            set.insert(key);
        }
        
        let mut add: u32 = 0;
        for value in values {
            if set.contains(value) {
                add = (add * 2).max(1);
            }
        }

        sum += add;
    }

    println!("Sum: {}", sum)
}