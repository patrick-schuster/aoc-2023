use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let size = content.lines().count();
    let data = content.replace("\n", "");
    let data = data.as_bytes();

    let mut sum = 0;
    for col in 0..size {
        let mut count = 0;
        let mut level = 0;
        for row in 0..size {
            match data[row * size + col] {
                b'#' => {
                    for i in 0..count {
                        sum += size - level - i;
                    }
                    level = row + 1;
                    count = 0;
                },
                b'O' => count += 1,
                _ => ()
            }
        }

        for i in 0..count {
            sum += size - level - i;
        }
    }

    println!("Sum: {}", sum);
}