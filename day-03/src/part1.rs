use std::fs;

fn main() {
    let mut content = fs::read_to_string("in.txt")
        .expect("Input file not found.");

    // Magic trick to skip bounds checks.
    content = format!(".{}.", content);

    let max_len = content.len();
    let mod_len = content.find("\n").unwrap() + 1;
    println!("{} {}", max_len, mod_len);
    let content: Vec<char> = content.replace("\n", "..").chars().collect();
    let mut len: usize = 0;
    let mut sum: u32 = 0;

    'outer: for (i, c) in content.iter().enumerate() {
        if c.is_ascii_digit() {
            len += 1;
            continue;
        }

        // Only perform check at the end of a number.
        if len > 0 {
            if i > mod_len {
                for k in (i - len - 1 - mod_len)..=i - mod_len {
                    if content[k] != '.' && !content[k].is_ascii_digit() {
                        sum += content[i - len..i].iter().collect::<String>().parse::<u32>().unwrap();
                        len = 0;
                        continue 'outer;
                    }
                }
            }

            if i + mod_len < max_len {
                for k in (i - len - 1 + mod_len)..=i + mod_len {
                    if content[k] != '.' && !content[k].is_ascii_digit() {
                        sum += content[i-len..i].iter().collect::<String>().parse::<u32>().unwrap();
                        len = 0;
                        continue 'outer;
                    }
                }
            }

            if content[i - len - 1] != '.' && !content[i - len - 1].is_ascii_digit() {
                sum += content[i - len..i].iter().collect::<String>().parse::<u32>().unwrap();
                len = 0;
                continue 'outer;
            }

            if content[i] != '.' && !content[i].is_ascii_digit() {
                sum += content[i - len..i].iter().collect::<String>().parse::<u32>().unwrap();
                len = 0;
                continue 'outer;
            }

            len = 0;
        }
    }

    println!("Sum: {}", sum);
}