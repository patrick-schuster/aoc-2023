use std::fs;

const SKIP: u8 = b'1';
const STRAIGHT: u8 = b'2';
const LEFT_UP: u8 = b'3';
const LEFT_DOWN : u8 = b'4';
const RIGHT_UP: u8 = b'5';
const RIGHT_DOWN: u8 = b'6';

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let len = content.lines()
        .next().unwrap().len();

    let mut content = content.replace("\n", "");
    let content = unsafe { content.as_bytes_mut() };
    let start: usize = content.iter()
        .position(|c| *c == b'S')
        .expect("No starting point found");

    // Replace the start point (its a special case).
    content[start] = {
        let left = matches!(content[start.checked_sub(1).unwrap_or(start)], b'-' | b'F' | b'L');
        let top = matches!(content[start.checked_sub(len).unwrap_or(start)], b'|' | b'F' | b'7');
        let right = matches!(content[start + 1], b'-' | b'J' | b'7');
        let bottom = matches!(content[start + len], b'|' | b'J' | b'L');

        if top && left { b'J' }
        else if top && right { b'L' }
        else if bottom && left { b'7' }
        else if bottom && right { b'F' }
        else if top && bottom { b'|' }
        else { b'-' }
    };

    let mut position = start;
    let mut previous = start;
    loop {
        // All possible matches for the current position.
        let matches = match content[position] {
            b'|' => (position - len, position + len),
            b'-' => (position - 1, position + 1),
            b'L' => (position - len, position + 1),
            b'J' => (position - len, position - 1),
            b'7' => (position + len, position - 1),
            b'F' => (position + len, position + 1),
            _ => (position, position),
        };

        // Convert to a special identifier to mark the path.
        content[position] = match content[position] {
            b'-' => SKIP,
            b'|' => STRAIGHT,
            b'J' => LEFT_UP,
            b'7' => LEFT_DOWN,
            b'L' => RIGHT_UP,
            b'F' => RIGHT_DOWN,
            c => c,
        };

        // Make sure to go forward, not backwards.
        let value = if matches.0 == previous || matches.0 == position {
            matches.1
        } else {
            matches.0
        };

        previous = position;
        position = value;
        if position == start {
            break;
        }
    }

    // Count the empty spaces in the grid by abusing math.
    let mut sum: u16 = 0;
    let mut prev: u8 = 0;
    let mut intersections: u8 = 0;
    for (i, c) in content.iter().enumerate() {
        if i % len == 0 {
            intersections = 0;
        }

        // Crossing a edge, look out for edge cases.
        if *c >= STRAIGHT && *c <= RIGHT_DOWN {
            if prev == RIGHT_UP && *c == LEFT_DOWN || prev == RIGHT_DOWN && *c == LEFT_UP {
                prev = *c;
                continue;
            } else if *c == RIGHT_DOWN || *c == RIGHT_UP || *c == STRAIGHT {
                prev = *c;
            }

            intersections += 1;
        } else if (*c < SKIP || *c > RIGHT_DOWN) && intersections & 1 == 1 {
            sum += 1;
        }
    }

    println!("Result: {}", sum);
}