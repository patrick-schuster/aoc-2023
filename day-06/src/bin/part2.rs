use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut lines = content.lines();
    let time: u64 = lines.next().unwrap()
        .split(": ").last().unwrap()
        .replace(" ", "").parse().unwrap();

    let distance: u64 = lines.next().unwrap()
        .split(": ").last().unwrap()
        .replace(" ", "").parse().unwrap();

    // Calculate area through midnight formula.
    // Min value (-b - sqrt(b^2 - 4ac)) / 2a)
    let min: f64 = (time as f64 - ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    let min: u64 = (min + 1.0).floor() as u64;

    // Max value (-b + sqrt(b^2 - 4ac)) / 2a)
    let max: f64 = (time as f64 + ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    let max: u64 = max.ceil() as u64;

    // Calculate range and multiply.
    let range: u64 = (max - min).into();
    println!("Result: {}", range);
}