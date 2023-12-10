use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let len = content.lines()
        .next().unwrap().len();

    let content = content.replace("\n", "");
    let content = content.as_bytes();

    let start = content.iter()
        .position(|c| *c == b'S')
        .expect("No starting point found");

    let mut position = start;
    let mut previous = start;
    let mut sum: u16 = 0;

    loop {
        let surroundings = match content[position] {
            b'|' => vec![position - len, position + len],
            b'-' => vec![position - 1, position + 1],
            b'L' => vec![position - len, position + 1],
            b'J' => vec![position - len, position - 1],
            b'7' => vec![position + len, position - 1],
            b'F' => vec![position + len, position + 1],
            b'S' => {
                let possibilities = [
                    (position.checked_sub(1).unwrap_or(position), &[b'-', b'F', b'L']),
                    (position.checked_sub(len).unwrap_or(position), &[b'|', b'F', b'7']),
                    (position + 1, &[b'-', b'J', b'7']),
                    (position + len, &[b'|', b'J', b'L']),
                ];

                possibilities.iter()
                    .filter(|(pos, chars)| chars.contains(&content[*pos]))
                    .map(|(p, _)| *p)
                    .collect()
            },
            _ => vec![],
        };

        for n in surroundings {
            if n != previous && n != position {
                sum += 1;
                previous = position;
                position = n;
                break;
            }
        }

        if position == start {
            break;
        }
    }

    println!("Steps: {}", sum / 2);
}