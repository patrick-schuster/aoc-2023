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
            while row - size > 0 && row + size < height {
                let first = &data[(row - size - 1) * width..(row - size) * width];
                let second = &data[(row + size) * width..(row + size + 1) * width];
                if first != second {
                    continue 'rows;
                }

                size += 1;
            }

            sum += row as u32 * 100;
            continue 'blocks;
        }

        // Check columns.
        'columns: for col in 1..width {
            let mut size = 0;
            while col - size > 0 && col + size < width {
                for row in 0..height {
                    let first = &data[row * width + col - size - 1..row * width + col - size];
                    let second = &data[row * width + col + size..row * width + col + size + 1];
                    if first != second {
                        continue 'columns
                    }
                }

                size += 1;
            }

            sum += col as u32;
            continue 'blocks
        }
    }

    println!("Sum: {}", sum);
}