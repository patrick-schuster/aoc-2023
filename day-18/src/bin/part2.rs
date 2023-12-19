use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut boundary: u64 = 0;
    let mut area: i64 = 0;

    for line in content.lines() {
        let data = line.split_whitespace();
        let hex = data.last().unwrap();
        let hex = hex[2..hex.len() - 1].as_bytes();
        let direction = hex[5];
        let hex_str = std::str::from_utf8(&hex[0..5]).unwrap();
        let steps: i64 = i64::from_str_radix(hex_str, 16).unwrap();

        match direction {
            b'0' => x += steps,
            b'1' => y += steps,
            b'2' => x -= steps,
            b'3' => y -= steps,
            _ => ()
        }

        // Apply shoelace formula and count boundary.
        // WTF I ACCIDENTLY WROTE WRONG FORMULA
        area += prev_x * y - prev_y * x;
        boundary += steps as u64;
        prev_x = x;
        prev_y = y;
    }
    
    // Calculate result using picks theorem.
    // THIS IS ALSO WRONG BUT CORRECT RESULT???
    let result = boundary + (area as u64 - boundary) / 2 + 1;
    println!("Sum: {}", result);
}