use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Input file does not exist.");

    let mut sum: u32 = 0;
    let lines = content.lines();
    for line in lines {
        let data: Vec<&str> = line.split(": ").collect();

        let mut red: u8 = 0;
        let mut green: u8 = 0;
        let mut blue: u8 = 0;

        let picks: Vec<&str> = data[1].split("; ").collect();
        for pick in picks {
            let cube_meta: Vec<&str> = pick.split(", ").collect();
            for cube in cube_meta {
                let cube_data: Vec<&str> = cube.split_whitespace().collect();
                let amount: u8 = cube_data[0].parse().expect("Could not parse cube amount.");

                match cube_data[1] {
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
