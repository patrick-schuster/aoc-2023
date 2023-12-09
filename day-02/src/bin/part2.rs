use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    let lines = content.lines();
    for line in lines {
        let data = line.split(": ");

        let mut red: u8 = 0;
        let mut green: u8 = 0;
        let mut blue: u8 = 0;

        let picks = data.last().unwrap().split("; ");
        for pick in picks {
            let cube_meta = pick.split(", ");
            for cube in cube_meta {
                let mut cube_data = cube.split_whitespace();
                let amount: u8 = cube_data.next().unwrap()
                    .parse().expect("Could not parse cube amount.");

                match cube_data.next().unwrap() {
                    "red" => red = red.max(amount),
                    "green" => green = green.max(amount),
                    "blue" => blue = blue.max(amount),
                    &_ => todo!()
                };
            }
        }

        sum += red as u32 * green as u32 * blue as u32;
    }

    println!("Sum: {}", sum);
}
