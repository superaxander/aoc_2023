use anyhow::Result;

use crate::common;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Rule {
    Accept,
    Reject,
    Gt(char, usize, String),
    Lt(char, usize, String),
    Jump(String),
}

#[derive(Copy, Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Part { x, m, a, s }
    }

    fn get(&self, c: char) -> usize {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Only Christmas is allowed"),
        }
    }

    fn get_mut(&mut self, c: char) -> &mut usize {
        match c {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!("Only Christmas is allowed"),
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/19.txt")?;

    let mut workflows = HashMap::new();
    let mut parts = VecDeque::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let (name, remaining) = line.split_once('{').unwrap();
        let list = &remaining[..remaining.len() - 1];
        if name.is_empty() {
            let mut part = Part::new(0, 0, 0, 0);
            for property in list.split(',') {
                let (name, value) = property.split_once('=').unwrap();
                *part.get_mut(name.chars().next().unwrap()) = value.parse::<usize>()?;
            }
            parts.push_back(("in".to_owned(), part));
        } else {
            let mut rules = Vec::new();
            for rule in list.split(',') {
                match rule {
                    "A" => rules.push(Rule::Accept),
                    "R" => rules.push(Rule::Reject),
                    rule => match rule.chars().nth(1).unwrap() {
                        '>' => {
                            let (num, destination) = &rule[2..].split_once(':').unwrap();
                            rules.push(Rule::Gt(
                                rule.chars().next().unwrap(),
                                num.parse()?,
                                (*destination).to_owned(),
                            ));
                        }
                        '<' => {
                            let (num, destination) = &rule[2..].split_once(':').unwrap();
                            rules.push(Rule::Lt(
                                rule.chars().next().unwrap(),
                                num.parse()?,
                                (*destination).to_owned(),
                            ));
                        }
                        _ => rules.push(Rule::Jump(rule.to_owned())),
                    },
                }
            }
            workflows.insert(name.to_owned(), rules);
        }
    }

    workflows.insert("A".to_owned(), vec![Rule::Accept]);
    workflows.insert("R".to_owned(), vec![Rule::Reject]);

    let mut solution_a = 0;
    while let Some((workflow, part)) = parts.pop_front() {
        for rule in &workflows[&workflow] {
            match rule {
                Rule::Accept => solution_a += part.sum(),
                Rule::Reject => {}
                Rule::Gt(c, threshold, destination) => {
                    if part.get(*c) > *threshold {
                        parts.push_back((destination.to_owned(), part));
                        break;
                    }
                }
                Rule::Lt(c, threshold, destination) => {
                    if part.get(*c) < *threshold {
                        parts.push_back((destination.to_owned(), part));
                        break;
                    }
                }
                Rule::Jump(destination) => {
                    parts.push_back((destination.to_owned(), part));
                    break;
                }
            }
        }
    }

    let solution_b = find_ranges(
        Part::new(1, 1, 1, 1),
        Part::new(4000, 4000, 4000, 4000),
        "in",
        &workflows,
    );

    Ok((solution_a, solution_b))
}

fn possibilities(min: Part, max: Part) -> usize {
    (max.x + 1 - min.x) * (max.m + 1 - min.m) * (max.a + 1 - min.a) * (max.s + 1 - min.s)
}

fn find_ranges(
    mut min: Part,
    mut max: Part,
    workflow: &str,
    workflows: &HashMap<String, Vec<Rule>>,
) -> usize {
    let mut acc = 0;
    for rule in &workflows[workflow] {
        match rule {
            Rule::Accept => return acc + possibilities(min, max),
            Rule::Reject => return acc,
            Rule::Gt(c, threshold, destination) => {
                if max.get(*c) <= *threshold {
                    continue;
                }
                let start = min.get_mut(*c);
                let old_start = *start;
                *start = old_start.max(*threshold + 1);
                acc += find_ranges(min, max, destination, workflows);
                *min.get_mut(*c) = old_start;
                *max.get_mut(*c) = *threshold;
            }
            Rule::Lt(c, threshold, destination) => {
                if min.get(*c) >= *threshold {
                    continue;
                }
                let end = max.get_mut(*c);
                let old_end = *end;
                *end = old_end.min(*threshold - 1);
                acc += find_ranges(min, max, destination, workflows);
                *min.get_mut(*c) = *threshold;
                *max.get_mut(*c) = old_end;
            }
            Rule::Jump(destination) => return acc + find_ranges(min, max, destination, workflows),
        }
    }
    acc
}
