use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    // Get the columns to expand.
    let len = content.lines().count();
    let mut columns: Vec<usize> = Vec::new();
    'outer: for i in 0..len {
        for line in content.lines() {
            if line.as_bytes()[i] == b'#' {
                continue 'outer;
            }
        }

        columns.push(i);
    }

    // Actual expansion.
    let mut increment = 0;
    let mut indices: Vec<(usize, usize)> = Vec::new();

    // Don't actually expand, just calculate.
    content.lines()
        .enumerate()
        .for_each(|(i, line)| {
            let mut n = line.to_string() + "\n";

            // Horizontal expansion.
            for (i, c) in columns.iter().enumerate() {
                n.insert(i + c, '.');
            }

            // Vertical expansion.
            if line.find('#').is_some() {
                n.bytes().enumerate().for_each(|(j, b)| {
                    if b == b'#' {
                        indices.push((j, i + increment));
                    }
                });
            } else {
                increment += 1;
            }
        });

    // Calculate all the distances.
    let mut sum: u32 = 0;
    for k in 0..indices.len() {
        for l in k + 1..indices.len() {
            sum += ((indices[k].0 as i32 - indices[l].0 as i32).abs()
                + (indices[k].1 as i32 - indices[l].1 as i32).abs()) as u32;
        }
    }

    println!("Sum: {}", sum);
}