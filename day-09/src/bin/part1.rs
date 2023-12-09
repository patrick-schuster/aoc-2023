use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: i32 = 0;
    for line in content.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let result = extrapolate(&numbers);
        sum += result;
    }

    println!("Sum: {}", sum);
}

fn extrapolate(numbers: &Vec<i32>) -> i32 {
    let mut seen = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] != seen {
            seen = numbers[i];
            break;
        }
    }

    if seen == numbers[0] {
        return seen;
    }

    let calc = numbers.windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i32>>();

    return extrapolate(&calc) + numbers[calc.len()];
}