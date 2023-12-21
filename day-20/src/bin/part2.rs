use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
enum Variant<'a> {
    Broadcast,
    Conjunction { memory: Vec<(&'a str, bool)> },
    FlipFlop { previous: bool, state: bool }
}

#[derive(Clone, PartialEq)]
struct Module<'a> {
    key: &'a str,
    destinations: Vec<&'a str>,
    variant: Variant<'a>
}

#[derive(Clone)]
struct Pulse<'a> {
    key: &'a str,
    state: bool
}

impl<'a> Module<'a> {
    fn handle(&mut self, pulse: Pulse<'a>) {
        match &mut self.variant {
            Variant::Conjunction { memory } => {
                let location = memory
                    .iter_mut()
                    .find(|(key, _)| *key == pulse.key);

                if let Some(target) = location {
                    target.1 = pulse.state;
                }
            }
            Variant::FlipFlop { previous, state } => {
                *previous = *state;
                if !pulse.state {
                    *state = !*state;
                }
            }
            _ => ()
        }
    }
}

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut modules: HashMap<&str, Module> = HashMap::new();
    for line in content.lines() {
        let mut parts = line.split(" -> ");
        let mut key = parts.next().unwrap();
        let destinations: Vec<&str> = parts
            .next()
            .unwrap()
            .split(", ")
            .collect();

        let variant = match key.as_bytes()[0] {
            b'%' => {
                key = &key[1..];
                Variant::FlipFlop { previous: false, state: false }
            }
            b'&' => {
                key = &key[1..];
                Variant::Conjunction { memory: vec![] }
            }
            _ => Variant::Broadcast
        };

        modules.insert(key, Module { key, destinations, variant });
    }
    
    // Create initial capacity for conjunctions.
    modules
        .values()
        .filter_map(|module| {
            if let Variant::Conjunction { .. } = &module.variant {
                Some((module.key, modules
                    .values()
                    .filter(|from| from.destinations.contains(&module.key))
                    .map(|from|from.clone())
                    .collect()))
            } else {
                None
            }
        })
        .collect::<Vec<(&str, Vec<Module>)>>()
        .iter()
        .for_each(|(key, inputs)| {
            if let Some(module) = modules.get_mut(key) {
                if let Variant::Conjunction { memory } = &mut module.variant {
                    for module in inputs {
                        memory.push((module.key, false));
                    }
                }
            }
        });

    let conjunction_key = modules
        .values()
        .find(|module| module.destinations.contains(&"rx"))
        .unwrap()
        .key;

    let conjunction = modules.get_key_value(conjunction_key).unwrap().1;
    let mut cycle = vec![0; match &conjunction.variant {
        Variant::Conjunction { memory } => memory.len(),
        _ => 0
    }];

    let mut deque: VecDeque<(&str, Option<Pulse>)> = VecDeque::new();
    for i in 1.. {
        deque.push_back(("broadcaster", None));
    
        while let Some(element) = deque.pop_front() {
            let key = element.0;
            if let Some(module) = modules.get_mut(key) {

                // Update state.
                if let Some(pulse) = element.1 {
                    module.handle(pulse);
                }

                // Push destinations.
                let destinations = module.destinations.clone();
                let pulse = match &module.variant {
                    Variant::Broadcast => Some(Pulse { key, state: false }),
                    Variant::Conjunction { memory } => {
                        let result = memory
                            .iter()
                            .all(|(_, state)| *state);

                        Some(Pulse { key, state: !result })
                    },
                    Variant::FlipFlop { previous, state } => if previous != state {
                        Some(Pulse { key, state: *state })
                    } else {
                        None
                    }
                };

                if let Some(pulse) = pulse {
                    destinations
                        .iter()
                        .for_each(|destination| {
                            if let Some(_) = modules.get(destination) {
                                deque.push_back((destination, Some(pulse.clone())));
                            }
                        });
                }
            }

            // Check for cycle element.
            if key == conjunction_key {
                if let Some(conjunction) = modules.get(key) {
                    match &conjunction.variant {
                        Variant::Conjunction { memory } => {
                            let position = memory
                                .iter()
                                .position(|(_, state)| *state);

                            if let Some(position) = position {
                                if cycle[position] == 0 {
                                    cycle[position] = i;
                                } else if cycle.iter().all(|x| *x != 0) {
                                    println!("Cycle: {}", cycle.iter().fold(1, |acc, x| lcm(acc, *x)));
                                    return;
                                }
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}