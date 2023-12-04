use std::fs;
use std::collections;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let input = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    let cards: Vec<&str> = input.split("\n").collect();
    let mut id: usize = 0;
    let mut sum: u32 = 0;
    let mut set: collections::HashSet<&str> = collections::HashSet::new();
    let mut vec: Vec<u32> = vec![0; cards.len()];
    
    for card in cards {
        set.clear();
        let values: Vec<&str> = card.split(" | ").collect();
        let keys: Vec<&str> = values[0].split(": ").collect::<Vec<&str>>()[1].split(" ").collect();
        let values: Vec<&str> = values[1].split(" ").collect();

        // Put one original card in the map and loop until no card of the id is left.
        vec[id] += 1;

        // Put all non-empty keys in the set.
        for key in &keys {
            if !key.is_empty() {
                set.insert(key);
            }
        }
        
        // Count number of matches.
        let mut count: usize = 0;
        for value in &values {
           if !value.is_empty() && set.contains(value) {
                count += 1;
            }
        }

        for i in 1..=count {
            vec[id + i] += vec[id];
        }

        sum += vec[id];
        id += 1;
    }

    println!("Sum: {}", sum)
}