use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/9.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let mut layers = vec![line
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];
        while layers.last().unwrap().iter().any(|n| *n != 0) {
            layers.push(
                layers
                    .last()
                    .unwrap()
                    .iter()
                    .copied()
                    .map_windows(|[n1, n2]| n2 - n1)
                    .collect::<Vec<i64>>(),
            );
        }
        let mut lower = 0;
        let mut upper = 0;
        for layer in layers.into_iter().rev().skip(1) {
            lower += layer.last().unwrap();
            upper = layer.first().unwrap() - upper;
        }
        solution_a += lower;
        solution_b += upper;
    }

    Ok((solution_a, solution_b))
}
