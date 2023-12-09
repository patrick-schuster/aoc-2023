use std::fs;
use std::collections;

fn main() {
    let input = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let cards: Vec<&str> = input.lines().collect();
    let mut id: usize = 0;
    let mut sum: u32 = 0;
    let mut set: collections::HashSet<&str> = collections::HashSet::new();
    let mut vec: Vec<u32> = vec![0; cards.len()];
    
    for card in cards {
        set.clear();

        let mut iter = card.split(" | ");
        let keys = iter.next().unwrap()
            .split(": ").last().unwrap()
            .split_whitespace();
        let values = iter.next().unwrap()
           .split_whitespace();

        // Put one original card in the map and loop until no card of the id is left.
        vec[id] += 1;

        // Put all non-empty keys in the set.
        for key in keys {
            set.insert(key);
        }
        
        // Count number of matches.
        let mut count: usize = 0;
        for value in values {
           if set.contains(value) {
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