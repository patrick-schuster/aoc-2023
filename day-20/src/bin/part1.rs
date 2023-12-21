use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
enum Variant<'a> {
    Broadcast,
    Conjunction { memory: Vec<(&'a str, bool)> },
    FlipFlop { previous: bool, state: bool }
}

#[derive(Clone)]
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

    let mut pulses: (usize, usize) = (0, 0);
    let mut deque: VecDeque<(&str, Option<Pulse>)> = VecDeque::new();
    for _ in 0..1000 {
        // Initial pulse.
        deque.push_back(("broadcaster", None));
        pulses.0 += 1;
    
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
                    Variant::Conjunction { memory } => Some(Pulse { 
                        key, 
                        state: !memory
                            .iter()
                            .all(|(_, state)| *state) }),
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

                            if pulse.state {
                                pulses.1 += 1;
                            } else {
                                pulses.0 += 1;
                            }
                        });
                }
            }
        }
    }

    println!("Pulses: {}", pulses.0 * pulses.1);
}