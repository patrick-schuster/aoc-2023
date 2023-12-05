use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    let mut sets = content.split("\n\n");
    let seeds = sets.next().expect("No seeds found.")
        .split(": ").collect::<Vec<&str>>()[1];

    let mut indices: Vec<u64> = Vec::new();
    let mut mapping: Vec<(u64, u64, u64)> = Vec::new();

    // For beginning, store all seeds in the indices list.
    for seed in seeds.split(" ") {
        indices.push(seed.parse::<u64>().unwrap());
    }

    // Now go through the other sets:
    // 1. Create a mapping for each seed (seed, range, destination).
    // 2. Map each seed to its destination and store to the indices list.
    for set in sets {
        println!("Using new set");
        mapping.clear();

        let mut lines = set.split("\n");
        let _ = lines.next();
        for line in lines {
            let mut iter = line.split_whitespace();
            let tuple: (u64, u64, u64) = (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap());

            mapping.push(tuple);
        }

        // Now for each seed, find its destination.
        let mut temp: Vec<u64> = Vec::new();
        'outer: for index in &indices {
            for tuple in &mapping {
                if index >= &tuple.1 && index <= &(tuple.1 + tuple.2) {
                    temp.push(tuple.0 + index - tuple.1);
                    continue 'outer;
                }
            }

            // No destination found, so destination is the seed itself.
            temp.push(*index);
        }

        // Replace the old seeds with the new ones.
        indices = temp;
    }

    // Get the min value.
    println!("Min: {}", indices.iter().min().unwrap());
}
