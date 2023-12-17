use std::fs;

const HORIZONTAL: u8 = 1;
const VERTICAL: u8 = 2;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false
        }
    }

    fn get_next(&self, index: usize, width: usize) -> Option<usize> {
        match self {
            Direction::Down => Some(index + width),
            Direction::Up => index.checked_sub(width),
            Direction::Right => if (index + 1) % width == 0 { None } else { Some(index + 1) }
            Direction::Left => {
                let result = index.checked_sub(1);
                if result.is_none() || result.unwrap() % width == width - 1 {
                    None
                } else {
                    result
                }
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let data = content.replace("\n", "");
    let data = data.as_bytes();
    let height = content
        .lines()
        .count();
    let width = data.len() / height;
    let mut sum = 0;
    for side in 0..2 {
        for i in 0..width {
            let mut visited = vec![0; data.len()];
            follow(data, &mut visited, i + (width - 1) * height * side, if side == 0 { Direction::Down } else { Direction::Up }, width, height);
        
            let current = visited
                .iter()
                .fold(0, |acc, x| acc + if *x != 0 { 1 } else { 0 });
            
            sum = sum.max(current);
        }

        for i in 0..height {
            let mut visited = vec![0; data.len()];
            follow(data, &mut visited, i + (width - 1) * height * side, if side == 0 { Direction::Right } else { Direction::Left }, width, height);
        
            let current = visited
                .iter()
                .fold(0, |acc, x| acc + if *x != 0 { 1 } else { 0 });
            
            sum = sum.max(current);
        }
    }

    println!("Sum: {}", sum);
}

fn follow(data: &[u8], visited: &mut Vec<u8>, index: usize, direction: Direction, width: usize, height: usize) {
    if index >= width * height {
        return
    }

    // Already seen, so already counted.
    let c = data[index];
    if c != b'/' && c != b'\\' {
        if direction.is_vertical() {
            if visited[index] & VERTICAL == 1 {
                return
            }
        } else if visited[index] & HORIZONTAL == 1 {
            return
        }
    }

    visited[index] |= if direction.is_vertical() { VERTICAL } else { HORIZONTAL };
    let mut direction = direction;

    match c {
        b'.' => (),
        b'-' => {
            if direction.is_vertical() {
                direction = Direction::Left;
                let next_direction = Direction::Right;
                let next = next_direction.get_next(index, width);
                if let Some(next_index) = next {
                    follow(data, visited, next_index, next_direction, width, height);
                }
            }
        }
        b'|' => {
            if !direction.is_vertical() {
                direction = Direction::Up;
                let next_direction = Direction::Down;
                let next = next_direction.get_next(index, width);
                if let Some(next_index) = next {
                    follow(data, visited, next_index, next_direction, width, height);
                }
            }
        }
        b'/' => direction = match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        },
        b'\\' => direction = match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        },
        _ => return
    }

    let next = direction.get_next(index, width);
    if next.is_none() {
        return
    }

    follow(data, visited, next.unwrap(), direction, width, height)
}