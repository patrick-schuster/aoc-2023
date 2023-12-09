use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    let lines = content.lines();
    for line in lines {
        let mut left: u8 = 0;
        let mut right: u8 = 0;
        let mut chars = line.chars();

        // Search from beginning until next digit.
        while let Some(c) = chars.next() {
            if left == 0 && c.is_ascii_digit() {
                left = c.to_digit(10).unwrap() as u8;
                break;
            }
        }
        
        // Search from end until next digit.
        while let Some(c) = chars.next_back() {
            if right == 0 && c.is_ascii_digit() {
                right = c.to_digit(10).unwrap() as u8;
                break;
            }
        }

        // Edge case if only one digit is found.
        if left > 0 && right == 0 {
            right = left
        }

        // Increment sum.
        sum += (left * 10 + right) as u32;
    }

    println!("Sum: {}", sum)
}