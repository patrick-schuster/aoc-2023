use std::fs;

const INCREMENT: usize = 1000000 - 1;

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
            
            // Expansion through math.
            let mut offset: usize = 0;
            let mut index: usize = 0;
            let mut key = columns[index];
            let mut flag = false;
            for (j, c) in line.bytes().enumerate() {
                if key == j {
                    offset += INCREMENT;
                    index += 1;

                    if index < columns.len() {
                        key = columns[index];
                    }
                }

                if c == b'#' {
                    indices.push((j + offset, i + increment));
                    flag = true;
                }
            }

            if !flag {
                increment += INCREMENT;
            }
        });

    // Calculate all the distances.
    let mut sum: u64 = 0;
    for k in 0..indices.len() {
        for l in k + 1..indices.len() {
            sum += ((indices[k].0 as i64 - indices[l].0 as i64).abs()
                + (indices[k].1 as i64 - indices[l].1 as i64).abs()) as u64;
        }
    }

    println!("Sum: {}", sum);
}