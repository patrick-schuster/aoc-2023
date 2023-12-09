use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("file not found");

    let mut sum: i32 = 0;
    let lines = content.lines();
    for line in lines {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let result = step_down(&numbers);
        sum += result;
        println!("{} -> {}", line, result);
    }

    println!("Sum: {}", sum);
}

fn step_down(numbers: &Vec<i32>) -> i32 {
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

    return numbers[0] - step_down(&calc);
}