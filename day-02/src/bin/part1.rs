use std::fs;

const RED_MAX: u8 = 12;
const GREEN_MAX: u8 = 13;
const BLUE_MAX: u8 = 14;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut sum: u32 = 0;
    let lines = content.lines();
    'game_loop: for line in lines {
        let mut data = line.split(": ");
        let id: u8 = data.next().unwrap()
            .split_whitespace().last().unwrap()
            .parse().expect("Could not parse game id.");

        let picks = data.next().unwrap().split("; ");
        for pick in picks {
            let cube_meta = pick.split(", ");
            for cube in cube_meta {
                let mut cube_data = cube.split_whitespace();
                let amount: u8 = cube_data.next().unwrap()
                    .parse().expect("Could not parse cube amount.");

                match cube_data.next().unwrap() {
                    "red" => if amount > RED_MAX { continue 'game_loop },
                    "green" => if amount > GREEN_MAX { continue 'game_loop },
                    "blue" => if amount > BLUE_MAX { continue 'game_loop },
                    &_ => todo!("Invalid color.")
                };
            }
        }

        sum += id as u32;
    }

    println!("Sum: {}", sum);
}
