use std::fs;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    steps: i32
}

impl Direction {
    fn from_char(b: u8) -> Direction {
        match b {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("Invalid input")
        }
    }
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in content.lines() {
        let mut data = line.split_whitespace();
        let direction = Direction::from_char(data.next().unwrap().bytes().nth(0).unwrap());
        let steps = data.next().unwrap().parse::<i32>().unwrap();
        match direction {
            Direction::Up => {
                vertical -= steps;
                min_y = min_y.min(vertical);
            }
            Direction::Down => {
                vertical += steps;
                max_y = max_y.max(vertical);
            }
            Direction::Left => {
                horizontal -= steps;
                min_x = min_x.min(horizontal);
            }
            Direction::Right => {
                horizontal += steps;
                max_x = max_x.max(horizontal);
            }
        }
        instructions.push(Instruction {direction, steps});
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut tracker: Vec<usize> = Vec::new();

    let mut sum = 0;
    let mut index = width * (min_y.abs() as usize) + (min_x.abs() as usize);
    tracker.push(index);
    for instruction in instructions {
        for _ in 0..instruction.steps as usize {
            match instruction.direction {
                Direction::Up => index -= width,
                Direction::Down => index += width,
                Direction::Left => index -= 1,
                Direction::Right => index += 1
            }

            tracker.push(index);
        }

        sum += instruction.steps;
    }

    for y in 0..height {
        let mut flag = false;
        let mut dir = Direction::Left;
        for x in 0..width {
            let index = y * width + x;
            if tracker.contains(&index) {
                let top = index.checked_sub(width);
                let bottom = index + width;
                let mut _dir = Direction::Right;
                if top.is_some() && tracker.contains(&top.unwrap()) {
                    _dir = if flag { Direction::Down } else { Direction::Up }
                }
                if bottom < width * height && tracker.contains(&bottom) {
                    _dir = if _dir == Direction::Down || _dir == Direction::Up {
                        Direction::Right
                    } else {
                        if flag { Direction::Up } else { Direction::Down }
                    }
                }

                if _dir == Direction::Up && dir == Direction::Up 
                || _dir == Direction::Down && dir == Direction::Down {

                } else if _dir != Direction::Left {
                    flag = !flag;
                }

                if _dir == Direction::Up || _dir == Direction::Down {
                    dir = _dir;
                }
            } else if flag {
                sum += 1;
            }
        }
    }

    println!("Sum: {}", sum);
}