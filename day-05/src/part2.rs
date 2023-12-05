use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    let mut sets = content.split("\n\n");
    let seeds = sets.next().expect("No seeds found.")
        .split(": ").collect::<Vec<&str>>()[1];

    let mut indices: Vec<(u64, u64)> = Vec::new();
    let mut mapping: Vec<(u64, u64, u64)> = Vec::new();

    // For beginning, store all seeds in the indices list.
    let values = seeds.split(" ").collect::<Vec<&str>>();
    for i in (0..values.len()).step_by(2) {
        indices.push((
            values[i].parse::<u64>().unwrap(),
            values[i + 1].parse::<u64>().unwrap()));
    }

    // Now go through the other sets:
    // 1. Create a mapping for each seed (seed, range, destination).
    // 2. Map each seed to its destination and store to the indices list.
    for set in sets {
        println!("Using new set");
        indices.sort_by(|a, b| a.0.cmp(&b.0));
        println!("Indices: {:?}", indices);
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

        // Sort the mapping by seed to check undefined areas.
        mapping.sort_by(|a, b| a.1.cmp(&b.1));

        // Now for each seed, find its destination.
        let mut temp: Vec<(u64, u64)> = Vec::new();
        'outer: for index in &indices {
            let mut current = index.0;
            let mut left_range = index.1;
            let mut end: u64 = 0;

            // Go through each mapping and perform calculations.
            for mapping in &mapping {

                // Go until first intersection.
                if index.0 > mapping.1 + mapping.2 {
                    continue;
                }

                // If the seed starts outside the range, do things.
                if current < mapping.1 {
                    if current + left_range < mapping.1 {
                        temp.push((current, left_range));
                        continue 'outer;
                    } else if current + left_range < mapping.1 + mapping.2 {
                        temp.push((current, mapping.1 - current));
                        temp.push((mapping.0, left_range - (mapping.1 - current)));
                        continue 'outer;
                    } else {
                        temp.push((mapping.0 + current - mapping.1, mapping.1 + mapping.2 - current));
                        left_range -= (mapping.1 - current) + mapping.2;
                        current = mapping.1 + mapping.2;
                        end = mapping.1 + mapping.2;
                        continue;
                    }
                } else {
                    if current + left_range < mapping.1 + mapping.2 {
                        temp.push((mapping.0 + (current - mapping.1), left_range));
                        continue 'outer;
                    } else {
                        temp.push((mapping.0 + current - mapping.1, mapping.1 + mapping.2 - current));
                        left_range -= mapping.1 + mapping.2 - current;
                        current = mapping.1 + mapping.2;
                        end = mapping.1 + mapping.2;
                        continue;
                    }
                }
            }

            if left_range > 0 {
                temp.push((current, left_range));
            }
        }

        // Replace the old seeds with the new ones.
        indices = temp;
    }

    // Get the min value.
    println!("Min: {}", indices.iter().min().unwrap().0);
}
