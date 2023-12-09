use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut lines = content.lines();
    let times = lines.next().unwrap()
        .split(": ").last().unwrap()
        .split_whitespace();

    let distances = lines.next().unwrap()
        .split(": ").last().unwrap()
        .split_whitespace();

    let mut result = 0;
    for (time, distance) in times.zip(distances) {
        let time: u16 = time.parse().unwrap();
        let distance: u16 = distance.parse().unwrap();

        // Calculate area through midnight formula.
        // Min value (-b - sqrt(b^2 - 4ac)) / 2a)
        let min: f32 = (time as f32 - ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
        let min: u16 = (min + 1.0).floor() as u16;

        // Max value (-b + sqrt(b^2 - 4ac)) / 2a)
        let max: f32 = (time as f32 + ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
        let max: u16 = max.ceil() as u16;
        
        // Calculate range and multiply.
        let range: u32 = (max - min).into();
        result = (result * range).max(range);
    }

    println!("Result: {}", result);
}