use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    'blocks: for block in content.split("\n\n") {
        let mut split = block.split("\n");
        let width = split.next().unwrap().len();
        let height = split.count() + 1;
        let data = block.replace("\n", "");

        // Check rows.
        'rows: for row in 1..height {
            let mut size = 0;
            let mut smudges: u8 = 0;

            while row - size > 0 && row + size < height {
                let first = &data[(row - size - 1) * width..(row - size) * width];
                let second = &data[(row + size) * width..(row + size + 1) * width];
                for (a, b) in first.chars().zip(second.chars()) {
                    if a != b {
                        smudges += 1;
                        if smudges > 1 {
                            continue 'rows;
                        }
                    }
                }
                
                size += 1;
            }

            size = 0;
            while row - size > 0 && row + size < height {
                let first = &data[(row - size - 1) * width..(row - size) * width];
                let second = &data[(row + size) * width..(row + size + 1) * width];
                if first != second {
                    sum += row as u32 * 100;
                    continue 'blocks
                }
                
                size += 1;
            }
        }

        // Check columns.
        'columns: for col in 1..width {
            let mut size = 0;
            let mut sludges: u8 = 0;

            while col - size > 0 && col + size < width {
                for row in 0..height {
                    let first = &data[row * width + col - size - 1..row * width + col - size];
                    let second = &data[row * width + col + size..row * width + col + size + 1];
                    if first != second {
                        sludges += 1;
                        if sludges > 1 {
                            continue 'columns
                        }
                    }
                }

                size += 1;
            }

            size = 0;
            while col - size > 0 && col + size < width {
                for row in 0..height {
                    let first = &data[row * width + col - size - 1..row * width + col - size];
                    let second = &data[row * width + col + size..row * width + col + size + 1];
                    if first != second {
                        sum += col as u32;
                        continue 'blocks
                    }
                }

                size += 1;
            }
        }
    }
    
    println!("Sum: {}", sum);
}