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
        indices.sort_by(|a, b| a.0.cmp(&b.0));
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

            // Go through each mapping and perform calculations.
            for mapping in &mapping {
                let dest = mapping.0;
                let start = mapping.1;
                let range = mapping.2;

                // Go until first intersection.
                if index.0 > start + range {
                    continue;
                }

                // If the seed starts outside the range, do things.
                if current < start {
                    if current + left_range < start {
                        temp.push((current, left_range));
                        continue 'outer;
                    } else if current + left_range < start + range {
                        temp.push((current, start - current));
                        temp.push((dest, left_range - (start - current)));
                        continue 'outer;
                    } else {
                        temp.push((dest + current - start, start + range - current));
                        left_range -= (start - current) + range;
                        current = start + range;
                        continue;
                    }
                } else {
                    if current + left_range < start + range {
                        temp.push((dest + (current - start), left_range));
                        continue 'outer;
                    } else {
                        temp.push((dest + current - start, start + range - current));
                        left_range -= start + range - current;
                        current = start + range;
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
