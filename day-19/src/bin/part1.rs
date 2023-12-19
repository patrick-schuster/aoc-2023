use std::fs;
use std::collections::HashMap;

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

struct Item {
    id: u8,
    amount: u16,
    cmp: bool,
    target: String
}

struct Workflow<'a> {
    items: Vec<Item>,
    last: &'a str
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");
    let mut chunks = content.split("\n\n");

    // First handle the workflows.
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for line in chunks.next().unwrap().lines() {
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

    // Now handle the inputs.
    let mut sum = 0u32;
    let start = workflows
        .get("in")
        .unwrap();
    for line in chunks.next().unwrap().lines() {
        let params = line[1..line.len() - 1]
            .split(",")
            .collect::<Vec<&str>>();
        
        if check_workflow(&workflows, start, &params) {
            sum += params
                .iter()
                .map(|x| x[2..].parse::<u32>().unwrap())
                .sum::<u32>();
        }
    }

    println!("Sum: {}", sum);
}

fn check_workflow(workflows: &HashMap<String, Workflow>, workflow: &Workflow, params: &Vec<&str>) -> bool {
    for item in &workflow.items {
        let param = params
            .iter()
            .find(|x| x.bytes().nth(0).unwrap() == item.id)
            .unwrap();

        let amount = param[2..].parse::<u16>().unwrap();
        if item.cmp {
            if amount > item.amount {
                return proxy(workflows, &item.target, params);
            }
        } else if amount < item.amount {
            return proxy(workflows, &item.target, params);
        }
    }

    proxy(workflows, workflow.last, params)
}

fn proxy(workflows: &HashMap<String, Workflow>, id: &str, params: &Vec<&str>) -> bool {
    match id {
        ACCEPTED => true,
        REJECTED => false,
        _ => check_workflow(workflows, workflows.get(id).unwrap(), params)
    }
}