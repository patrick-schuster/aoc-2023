use std::fs;

const RED_MAX: u8 = 12;
const GREEN_MAX: u8 = 13;
const BLUE_MAX: u8 = 14;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Input file does not exist.");

    let mut sum: u32 = 0;
    let lines = content.lines();
    'game_loop: for line in lines {
        let data: Vec<&str> = line.split(": ").collect();
        let meta: Vec<&str> = data[0].split_whitespace().collect();
        let id: u8 = meta[1].parse().expect("Could not parse game id.");

        let picks: Vec<&str> = data[1].split("; ").collect();
        for pick in picks {
            let cube_meta: Vec<&str> = pick.split(", ").collect();
            for cube in cube_meta {
                let cube_data: Vec<&str> = cube.split_whitespace().collect();
                let amount: u8 = cube_data[0].parse().expect("Could not parse cube amount.");

                match cube_data[1] {
                    "red" => {
                        if amount > RED_MAX {
                            println!("Too much read: {}", amount);
                            continue 'game_loop
                        }
                    },
                    "green" => {
                        if amount > GREEN_MAX {
                            println!("Too much read: {}", amount);
                            continue 'game_loop
                        }
                    },
                    "blue" => {
                        if amount > BLUE_MAX {
                            println!("Too much read: {}", amount);
                            continue 'game_loop
                        }
                    },
                    &_ => todo!()
                };
            }
        }

        sum += id as u32;
    }

    println!("Sum: {}", sum);
}
