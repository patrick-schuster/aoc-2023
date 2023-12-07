use std::fs;

struct Mapping {
    dst: u64,
    src: u64,
    range: u64
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    let mut sets = content.split("\n\n");
    let seeds = sets.next().expect("No seeds found.")
        .split(": ").collect::<Vec<&str>>()[1];

    let mut indices: Vec<u64> = Vec::new();
    let mut mappings: Vec<Mapping> = Vec::new();

    // For beginning, store all seeds in the indices list.
    for seed in seeds.split(" ") {
        indices.push(seed.parse::<u64>().unwrap());
    }

    // Now go through the other sets:
    // 1. Create a mapping for each seed (seed, range, destination).
    // 2. Map each seed to its destination and store to the indices list.
    for set in sets {
        mappings.clear();

        let mut lines = set.split("\n");
        let _ = lines.next();
        for line in lines {
            let mut iter = line.split_whitespace();
            let mapping = Mapping {
                dst: iter.next().unwrap().parse().unwrap(),
                src: iter.next().unwrap().parse().unwrap(),
                range: iter.next().unwrap().parse().unwrap()
            };

            mappings.push(mapping);
        }

        // Now for each seed, find its destination.
        let mut temp: Vec<u64> = Vec::new();
        'outer: for index in &indices {
            for m in &mappings {
                if *index >= m.src && *index <= m.src + m.range {
                    temp.push(m.dst + index - m.src);
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
