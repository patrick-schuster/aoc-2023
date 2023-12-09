use std::fs;

fn main() {
    let mut content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    // Magic trick to skip bounds checks.
    content = format!(".{}.", content);

    let mod_len = content.find("\n").unwrap() + 1;
    let content: Vec<char> = content.replace("\n", "..").chars().collect();
    let max_len = content.len();

    let mut sum: u32 = 0;

    for (i, c) in content.iter().enumerate() {
        if *c == '*' {
            let mut parts = 0;
            let mut index_part: [usize; 2] = [0, 0];

            // Check numbers to left and right.
            if content[i - 1].is_ascii_digit() {
                index_part[0] = i - 1;
                parts += 1;
            }

            if content[i + 1].is_ascii_digit() {
                index_part[parts] = i + 1;
                parts += 1;
            }

            // Check numbers above.
            if i > mod_len {
                let tl = content[i - mod_len - 1].is_ascii_digit();
                let tr = content[i - mod_len + 1].is_ascii_digit();
                let tm = content[i - mod_len].is_ascii_digit();

                if tm {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i - mod_len;
                    parts += 1;
                } else if tl && !tr {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i - mod_len - 1;
                    parts += 1;
                } else if !tl && tr {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i - mod_len + 1;
                    parts += 1;
                } else if tl && tr && parts < 2 {
                    if parts > 0 {
                        continue;
                    }
                    index_part[parts] = i - mod_len - 1;
                    index_part[parts + 1] = i - mod_len + 1;
                    parts += 2;
                }
            }

            // Check numbers below.
            if i + mod_len < max_len {
                let bl = content[i + mod_len - 1].is_ascii_digit();
                let br = content[i + mod_len + 1].is_ascii_digit();
                let bm = content[i + mod_len].is_ascii_digit();

                if bm {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i + mod_len;
                    parts += 1;
                } else if bl && !br {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i + mod_len - 1;
                    parts += 1;
                } else if !bl && br {
                    if parts == 2 {
                        continue;
                    }
                    index_part[parts] = i + mod_len + 1;
                    parts += 1;
                } else if bl && br {
                    if parts > 0 {
                        continue;
                    }

                    index_part[parts] = i + mod_len - 1;
                    index_part[parts + 1] = i + mod_len + 1;
                    parts += 2;
                }
            }

            if parts == 2 {
                // Find both numbers by index parts.
                // Go to left and right until it hits something else than a digit.
                let mut digits = [0, 0];
                for k in 0..2 {
                    let mut left = index_part[k];
                    let mut right = index_part[k];
                    while content[left].is_ascii_digit() {
                        left -= 1;
                    }
                    while content[right].is_ascii_digit() {
                        right += 1;
                    }

                    digits[k] = content[left + 1..right].iter().collect::<String>().parse::<u32>().unwrap();
                }

                sum += digits[0] * digits[1];
            }
        }
    }

    println!("Sum: {}", sum);
}