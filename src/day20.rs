use anyhow::Result;

use crate::common;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/20.txt")?;

    let mut modules = HashMap::new();
    let mut inputs: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (name, destinations) = line.split_once(" -> ").unwrap();
        let destinations: Vec<String> = destinations.split(", ").map(ToOwned::to_owned).collect();
        for d in &destinations {
            inputs
                .entry(d.to_owned())
                .or_default()
                .insert(if name == "broadcaster" {
                    "broadcaster".to_owned()
                } else {
                    name[1..].to_owned()
                });
        }
        match name.chars().next().unwrap() {
            '%' => modules.insert(name[1..].to_owned(), Module::FlipFlop(false, destinations)),
            '&' => modules.insert(
                name[1..].to_owned(),
                Module::Conjunction(HashMap::new(), destinations),
            ),
            _ => modules.insert(name.to_owned(), Module::Broadcaster(destinations)),
        };
    }

    for (name, module) in &mut modules {
        if let Module::Conjunction(map, _) = module {
            for input in &inputs[name] {
                map.insert(input.to_owned(), false);
            }
        }
    }

    let mut critical_origins = Vec::new();
    for input in &inputs["rx"] {
        for input in &inputs[input] {
            critical_origins.push(input.to_owned());
        }
    }

    let mut queue = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut solution_a = 0;
    let mut last_seen = HashMap::new();
    let mut periods = HashMap::new();

    'outer: for i in 0.. {
        queue.push_back((false, "broadcaster".to_owned(), "button".to_owned()));

        while let Some((pulse, module, origin)) = queue.pop_front() {
            if pulse {
                high_pulses += 1;
                if critical_origins.contains(&origin) {
                    if let Some(prev) = last_seen.insert(origin.clone(), i) {
                        periods.insert(origin.clone(), i - prev);
                        if periods.len() == critical_origins.len() {
                            break 'outer;
                        }
                    }
                }
            } else {
                low_pulses += 1;
            }

            match modules.get_mut(&module) {
                Some(Module::Broadcaster(destinations)) => {
                    for d in destinations {
                        queue.push_back((pulse, d.to_owned(), module.clone()));
                    }
                }
                Some(Module::FlipFlop(state, destinations)) => {
                    if !pulse {
                        *state = !*state;
                        for d in destinations {
                            queue.push_back((*state, d.to_owned(), module.clone()));
                        }
                    }
                }
                Some(Module::Conjunction(state, destinations)) => {
                    state.insert(origin, pulse);
                    let out = !state.values().all(|b| *b);
                    for d in destinations {
                        queue.push_back((out, d.to_owned(), module.clone()));
                    }
                }
                _ => {}
            }
        }
        if i == 999 {
            solution_a = low_pulses * high_pulses;
        }
    }

    let solution_b = periods.values().product();

    Ok((solution_a, solution_b))
}

#[derive(Clone, Debug)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, bool>, Vec<String>),
}
