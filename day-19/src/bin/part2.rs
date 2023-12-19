use std::fs;
use std::collections::HashMap;

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const MIN_CMP: u16 = 1;
const MAX_CMP: u16 = 4000;

struct Item {
    id: u8,
    amount: u16,
    cmp: bool,
    target: String
}

#[derive(Clone)]
struct Input {
    id: u8,
    min: u16,
    max: u16
}

struct Workflow<'a> {
    items: Vec<Item>,
    last: &'a str
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    // First handle the workflows.
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for line in content.split("\n\n").next().unwrap().lines() {
        let mut split = line.split("{");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap();
        let iter = value[0..value.len() - 1].split(",");
        let mut items: Vec<Item> = Vec::new();
        let mut last = "";
        for item in iter {
            let mut split = item.split(":");
            if split.clone().count() > 1 {
                let eq = split.next().unwrap();
                let target = split.next().unwrap().to_string();
                let id = eq.bytes().nth(0).unwrap();
                let cmp = eq.bytes().nth(1).unwrap() == b'>';
                let amount = eq[2..].parse::<u16>().unwrap();
                items.push(Item { id, amount, cmp, target });
            } else {
                last = split.next().unwrap();
            }
        }
    
        workflows.insert(key, Workflow { items, last });
    }

    // Run the range simulation.
    let mut params: Vec<Input> = [b'x', b'm', b'a', b's']
        .iter()
        .map(|x| Input { id: *x, min: MIN_CMP, max: MAX_CMP })
        .collect();
    let sum = check_workflow(&workflows, workflows.get("in").unwrap(), &mut params);
    println!("Sum: {}", sum);
    println!("Valid: {}", sum < 4000*4000*4000*4000)
}

fn check_workflow(workflows: &HashMap<String, Workflow>, workflow: &Workflow, params: &mut Vec<Input>) -> u64 {
    let mut sum: u64 = 0;
    for item in &workflow.items {
        let mut params_cl = params.clone();
        let param = params_cl
            .iter()
            .find(|x| x.id == item.id)
            .unwrap();

        if item.cmp {
            if param.min > item.amount {
                return sum + proxy(workflows, &item.target, &mut params_cl);
            } else if param.min < item.amount {
                if param.max > item.amount {
                    *params = params_cl
                        .iter()
                        .map(|x| {
                            if x.id == item.id {
                                Input { id: x.id, min: x.min, max: item.amount }
                            } else {
                                x.clone()
                            }
                        }).collect::<Vec<Input>>();
                    let mut new_params = params_cl
                        .iter()
                        .map(|x| {
                            if x.id == item.id {
                                Input { id: x.id, min: item.amount + 1, max: x.max }
                            } else {
                                x.clone()
                            }
                        })
                        .collect::<Vec<Input>>();
                    sum += proxy(workflows, &item.target, &mut new_params);
                } else {
                    continue
                }
            }
        } else {
            if param.max < item.amount {
                return sum + proxy(workflows, &item.target, &mut params_cl);
            } else if param.max > item.amount {
                if param.min < item.amount {
                    let mut new_params = params_cl
                        .iter()
                        .map(|x| {
                            if x.id == item.id {
                                Input { id: x.id, min: x.min, max: item.amount - 1 }
                            } else {
                                x.clone()
                            }
                        })
                        .collect::<Vec<Input>>();
                    *params = params_cl
                        .iter()
                        .map(|x| {
                            if x.id == item.id {
                                Input { id: x.id, min: item.amount, max: x.max }
                            } else {
                                x.clone()
                            }
                        })
                        .collect::<Vec<Input>>();
                    sum += proxy(workflows, &item.target, &mut new_params);
                } else {
                    continue
                }
            } 
        }
    }

    sum + proxy(workflows, workflow.last, params)
}

fn proxy(workflows: &HashMap<String, Workflow>, id: &str, params: &mut Vec<Input>) -> u64 {
    match id {
        ACCEPTED => params
            .iter()
            .fold(1, |acc, x| acc * (x.max - x.min + 1) as u64),
        REJECTED => 0,
        _ => check_workflow(workflows, workflows.get(id).unwrap(), params)
    }
}