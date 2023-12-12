use anyhow::Result;

use crate::common;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (map, legend) = line.split_once(' ').unwrap();

        let map = map.chars().collect::<Vec<char>>();
        let mut legend = legend
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut visited_map = HashMap::new();
        solution_a += count_possibilities(&map, &mut legend, false, &mut visited_map).unwrap();
        visited_map.clear();
        let mut new_map = map.clone();
        for _ in 0..4 {
            new_map.push('?');
            new_map.extend(map.iter().copied());
        }
        solution_b +=
            count_possibilities(&new_map, &mut legend.repeat(5), false, &mut visited_map).unwrap();
    }

    Ok((solution_a, solution_b))
}

fn count_possibilities(
    map: &[char],
    legend: &mut [usize],
    in_chain: bool,
    visited_map: &mut HashMap<(usize, usize, usize), std::result::Result<usize, ()>>,
) -> std::result::Result<usize, ()> {
    if map.is_empty() && (legend.is_empty() || (legend.len() == 1 && legend[0] == 0)) {
        return Ok(1);
    } else if map.is_empty() {
        return Err(());
    } else if legend.is_empty() {
        if map.iter().any(|c| *c == '#') {
            return Err(());
        }
        return Ok(1);
    }

    if map[0] == '.' {
        if in_chain {
            if legend[0] == 0 {
                count_possibilities(&map[1..], &mut legend[1..], false, visited_map)
            } else {
                Err(())
            }
        } else {
            count_possibilities(&map[1..], legend, false, visited_map)
        }
    } else if map[0] == '#' {
        if legend[0] == 0 {
            Err(())
        } else {
            legend[0] -= 1;
            let result = count_possibilities(&map[1..], legend, true, visited_map);
            legend[0] += 1;
            result
        }
    } else {
        // if map[0] == '?'
        if in_chain {
            if legend[0] == 0 {
                count_possibilities(&map[1..], &mut legend[1..], false, visited_map)
            } else {
                let index = (map.len(), legend.len(), legend[0]);
                if visited_map.contains_key(&index) {
                    return visited_map[&index];
                }
                legend[0] -= 1;
                let result = count_possibilities(&map[1..], legend, true, visited_map);
                legend[0] += 1;
                visited_map.insert(index, result);
                result
            }
        } else if legend[0] == 0 {
            count_possibilities(&map[1..], &mut legend[1..], false, visited_map)
        } else {
            let index = (map.len(), legend.len(), legend[0]);
            if visited_map.contains_key(&index) {
                return visited_map[&index];
            }
            let working_case = count_possibilities(&map[1..], legend, false, visited_map);
            legend[0] -= 1;
            let broken_case = count_possibilities(&map[1..], legend, true, visited_map);
            legend[0] += 1;
            let mut sum = 0;
            if let Ok(a) = working_case {
                sum += a;
            }
            if let Ok(b) = broken_case {
                sum += b;
            }
            if sum == 0 {
                visited_map.insert(index, Err(()));
                Err(())
            } else {
                visited_map.insert(index, Ok(sum));
                Ok(sum)
            }
        }
    }
}
