use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let len = content.lines()
        .next().unwrap().len();

    let mut content = content.replace("\n", "");
    let content = unsafe { content.as_bytes_mut() };
    let start = content.iter()
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

    let mut sum: u16 = 0;
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

        // Make sure to go forward, not backwards.
        let value = if matches.0 == previous || matches.0 == position {
            matches.1
        } else {
            matches.0
        };

        sum += 1;
        previous = position;
        position = value;
        if position == start {
            break;
        }
    }

    println!("Steps: {}", sum / 2);
}