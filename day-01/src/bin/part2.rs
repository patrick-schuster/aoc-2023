use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine"
];

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    let lines = content.lines();
    for line in lines {
        let mut left: u8 = 0;
        let mut right: u8 = 0;
        let chars: Vec<char> = line.chars().collect();

        // Search from beginning until next digit.
        for (i, c) in chars.iter().enumerate() {
            if left == 0 {
                if c.is_ascii_digit() {
                    left = c.to_digit(10).unwrap() as u8;
                    break;
                }

                let num_left = check_left(&chars, i);
                if num_left > 0 {
                    left = num_left;
                    break;
                }
            }
        }
        
        // Search from end until next digit, but let i start from end without reversing the whole string
        for i in (0..chars.len()).rev() {
            if right == 0 {
                let c = chars[i];
                if c.is_ascii_digit() {
                    right = c.to_digit(10).unwrap() as u8;
                    break;
                }

                let num_right = check_right(&chars, i);
                if num_right > 0 {
                    right = num_right;
                    break;
                }
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

fn check_left(chars: &Vec<char>, i: usize) -> u8 {
    let word: String = chars[0.max(i.saturating_sub(4))..=i].iter().collect();
    for (i, num) in NUMBERS.iter().enumerate() {
        if word.ends_with(num) {
            return (i + 1) as u8;
        }
    }
    0
}

fn check_right(chars: &Vec<char>, i: usize) -> u8 {
    let word: String = chars[i..chars.len().min(i + 5)].iter().collect();
    for (i, num) in NUMBERS.iter().enumerate() {
        if word.starts_with(num) {
            return (i + 1) as u8;
        }
    }
    0
}