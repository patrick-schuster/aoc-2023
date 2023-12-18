use std::fs;

const CHAIN_LMT: u8 = 3;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

#[derive(Clone)]
struct Node {
    previous: Option<Box<Node>>,
    index: usize,
    cost: u32,
    chain: u8,
    direction: Direction
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.chain == other.chain && self.direction == other.direction
    }
}

impl Node {
    fn new(previous: Option<Box<Node>>, index: usize, cost: u32, chain: u8, direction: Direction) -> Self {
        Self {
            previous,
            index,
            cost,
            chain,
            direction
        }
    }
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let data = content.replace("\n", "");
    let data = data.as_bytes()
        .iter()
        .map(|x| *x - b'0')
        .collect::<Vec<u8>>();
    let height = content
        .lines()
        .count();
    let width = data.len() / height;
    
    // Sets for path finding.
    let mut open_set: Vec<Node> = Vec::new();
    let mut closed_set: Vec<Node> = Vec::new();

    // Add the initial node.
    open_set.push(Node::new(None, 0, 0, 1, Direction::None));

    // Loop until no more open nodes left.
    while !open_set.is_empty() {
        max = max.max(open_set.len());
        let min_cost_index = find_min_index(&open_set);
        let current = open_set.remove(min_cost_index);
        closed_set.push(current.clone());

        // Stop the path finding.
        if current.index == width * height - 1 {
            println!("Finished!");
            println!("Shortest path: {}", current.cost);
            break
        }

        let mut next_nodes = Vec::with_capacity(3);
        let index = current.index;
        let left = index.checked_sub(1);
        let top = index.checked_sub(width);
        let right = index + 1;
        let bottom = index + width;
        if left.is_some() && left.unwrap() % width != width - 1 && current.direction != Direction::Right {
            next_nodes.push(Node {
                previous: Some(Box::new(current.clone())),
                index: left.unwrap(),
                cost: current.cost + data[left.unwrap()] as u32,
                chain: if current.direction == Direction::None || current.direction == Direction::Left {
                    current.chain + 1
                } else {
                    1
                },
                direction: Direction::Left
            });
        }

        if top.is_some() && current.direction != Direction::Down {
            next_nodes.push(Node {
                previous: Some(Box::new(current.clone())),
                index: top.unwrap(),
                cost: current.cost + data[top.unwrap()] as u32,
                chain: if current.direction == Direction::None || current.direction == Direction::Up {
                    current.chain + 1
                } else {
                    1
                },
                direction: Direction::Up
            });
        }

        if right % width != 0 && current.direction != Direction::Left {
            next_nodes.push(Node {
                previous: Some(Box::new(current.clone())),
                index: right,
                cost: current.cost + data[right] as u32,
                chain: if current.direction == Direction::None || current.direction == Direction::Right {
                    current.chain + 1
                } else {
                    1
                },
                direction: Direction::Right
            });
        }

        if bottom < width * height && current.direction != Direction::Up {
            next_nodes.push(Node {
                previous: Some(Box::new(current.clone())),
                index: bottom,
                cost: current.cost + data[bottom] as u32,
                chain: if current.direction == Direction::None || current.direction == Direction::Down {
                    current.chain + 1
                } else {
                    1
                },
                direction: Direction::Down
            });
        }

        for next in next_nodes {
            if next.chain > CHAIN_LMT {
                continue;
            }

            if !closed_set.contains(&next) {
                if let Some(index) = open_set.iter().position(|x| x == &next) {
                    if open_set[index].cost > next.cost {
                        open_set[index] = next;
                    }
                } else {
                    open_set.push(next);
                }
            }
        }
    }
}

fn find_min_index(nodes: &Vec<Node>) -> usize {
    let min_cost = nodes
        .iter()
        .map(|x| x.cost)
        .min()
        .unwrap();
    nodes
        .iter()
        .position(|x| x.cost == min_cost)
        .unwrap()
}