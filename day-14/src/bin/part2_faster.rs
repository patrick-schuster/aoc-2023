use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let size = content.lines().count();
    let mut data = content.replace("\n", "");
    let data = unsafe { data.as_bytes_mut() };
    let mut states: Vec<Vec<u8>> = Vec::new();
    for i in 1.. {
        cycle(data, size);

        let state = data.to_vec();
        let index = states
            .iter()
            .position(|v| v == &state);

        if index.is_some() {
            let value = index.unwrap();
            let access = value + (1000000000 - i) % (i - value - 1);
            let data = &states[access];
        
            // Calculate the sum without tilting forward.
            let mut sum = 0;
            for col in 0..size {
                for row in 0..size {
                    match data[row * size + col] {
                        b'O' => sum += size - row,
                        _ => ()
                    }
                }
            }
        
            println!("Sum: {}", sum);
            break
        }

        states.push(state);
    }
}

fn cycle(data: &mut [u8], size: usize) {
    // Tilt left-forward.
    for col in 0..size {
        let mut all = 0;
        let mut count = 0;
        let mut level = 0;
        for row in 0..size {
            match data[row * size + col] {
                b'#' => {
                    for i in 0..count {
                        let mut x = col;
                        while x > 0 && data[(level + i) * size + x - 1] == b'.' {
                            x -= 1;
                        }

                        data[(level + i) * size + col] = b'.';
                        data[(level + i) * size + x] = b'O';
                    }
                    for i in count..count + all {
                        data[(level + i) * size + col] = b'.';
                    }
                    level = row + 1;
                    count = 0;
                    all = 0;
                },
                b'O' => count += 1,
                b'.' => all += 1,
                _ => ()
            }
        }

        for i in 0..count {
            let mut x = col;
            while x > 0 && data[(level + i) * size + x - 1] == b'.' {
                x -= 1;
            }

            data[(level + i) * size + col] = b'.';
            data[(level + i) * size + x] = b'O';
        }
        for i in count..count + all {
            data[(level + i) * size + col] = b'.';
        }
    }

    // Tilt right-downwards.
    for col in (0..size).rev() {
        let mut all = 0;
        let mut count = 0;
        let mut level = size - 1;
        for row in (0..size).rev() {
            match data[row * size + col] {
                b'#' => {
                    for i in 0..count {
                        let mut x = col;
                        while x < size - 1 && data[(level - i) * size + x + 1] == b'.' {
                            x += 1;
                        }

                        data[(level - i) * size + col] = b'.';
                        data[(level - i) * size + x] = b'O';
                    }
                    for i in count..count + all {
                        data[(level - i) * size + col] = b'.';
                    }
                    level = row.checked_sub(1).unwrap_or(0);
                    count = 0;
                    all = 0;
                },
                b'O' => count += 1,
                b'.' => all += 1,
                _ => ()
            }
        }

        for i in 0..count {
            let mut x = col;
            while x < size - 1 && data[(level - i) * size + x + 1] == b'.' {
                x += 1;
            }

            data[(level - i) * size + col] = b'.';
            data[(level - i) * size + x] = b'O';
        }
        for i in count..count + all {
            data[(level - i) * size + col] = b'.';
        }
    }
}