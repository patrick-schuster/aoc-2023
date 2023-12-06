use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut lines = content.lines();
    let times = lines.next().unwrap()
        .split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace().collect::<Vec<&str>>();

    let distances = lines.next().unwrap()
        .split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace().collect::<Vec<&str>>();

    let mut result: u64 = 0;
    for i in 0..times.len() {
        let time: u16 = times[i].parse().unwrap();
        let distance: u16 = distances[i].parse().unwrap();

        // Calculate area through midnight formula.
        // Min value (-b - sqrt(b^2 - 4ac)) / 2a)
        let min: f32 = (time as f32 - ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
        let min: u16 = (min + 1.0).floor() as u16;

        // Max value (-b + sqrt(b^2 - 4ac)) / 2a)
        let max: f32 = (time as f32 + ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
        let max: u16 = max.ceil() as u16;
        
        // Calculate range and multiply.
        let range: u32 = (max - min).into();
        println!("Range: {}", range);
        result = (result * range).max(range);
    }

    println!("Result: {}", result);
}