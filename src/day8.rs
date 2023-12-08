use anyhow::Result;

use crate::common;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let mut lines = common::read_lines("inputs/8.txt")?;

    let mut solution_a = 0;

    let moves = lines.next().unwrap()?.trim().chars().collect::<Vec<char>>();
    let mut map = HashMap::new();

    for line in lines.skip(1) {
        let line = line?;
        let line = line.trim();
        let (node, remaining) = line.split_once(" = (").unwrap();
        let (left, remaining) = remaining.split_once(", ").unwrap();
        let right = &remaining[..remaining.len() - 1];
        map.insert(node.to_owned(), (left.to_owned(), right.to_owned()));
    }

    let mut current = "AAA";
    let mut index = 0;
    while current != "ZZZ" {
        if moves[index] == 'L' {
            current = &map[current].0;
        } else {
            current = &map[current].1;
        }
        solution_a += 1;
        index = (index + 1) % moves.len();
    }

    let mut iteration = 0;
    let mut current = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<&String>>();
    let mut last_seen = vec![0; current.len()];
    let mut periods = vec![0; current.len()];
    let mut index = 0;
    'outer: loop {
        for (i, loc) in current.iter_mut().enumerate() {
            if periods[i] != 0 {
                continue;
            }
            if moves[index] == 'L' {
                *loc = &map[loc.as_str()].0;
            } else {
                *loc = &map[loc.as_str()].1;
            }
            if loc.ends_with('Z') {
                if last_seen[i] != 0 {
                    periods[i] = iteration - last_seen[i];
                    if periods.iter().all(|p| *p != 0) {
                        break 'outer;
                    }
                }
                last_seen[i] = iteration;
            }
        }
        iteration += 1;
        index = (index + 1) % moves.len();
    }

    let solution_b = periods.into_iter().reduce(lcm).unwrap();

    Ok((solution_a, solution_b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
