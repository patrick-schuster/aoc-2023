use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut x = 0;
    let mut y = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut boundary = 0;
    let mut area = 0;
    for line in content.lines() {
        let mut data = line.split_whitespace();
        let direction = data.next().unwrap().bytes().nth(0).unwrap();
        let steps = data.next().unwrap().parse::<i32>().unwrap();
        
        match direction {
            b'U' => y -= steps,
            b'D' => y += steps,
            b'L' => x -= steps,
            b'R' => x += steps,
            _ => ()
        }

        // Apply shoelace formula and count boundary.
        // WTF I ACCIDENTLY WROTE WRONG FORMULA
        area += prev_x * y - prev_y * x;
        boundary += steps;
        prev_x = x;
        prev_y = y;
    }

    // Calculate result using picks theorem.
    // THIS IS ALSO WRONG BUT CORRECT RESULT???
    let result = boundary + (area - boundary) / 2 + 1;
    println!("Sum: {}", result);
}