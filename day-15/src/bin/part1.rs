use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.

    let mut sum = 0;
    for s in content.split(",") {
        let mut value = 0;
        for b in s.bytes() {
            value += b as u32;
            value *= 17;
            value %= 256;
        }
        sum += value;
    }

    println!("Sum: {}", sum);
}