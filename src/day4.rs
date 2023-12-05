use anyhow::Result;

use crate::common;
use std::collections::HashSet;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/4.txt")?;

    let mut solution_a = 0;

    let mut cards = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let (_, remaining) = line.split_once(": ").unwrap();
        let (winning_half, owned_half) = remaining.split_once('|').unwrap();
        let winning = winning_half
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<HashSet<usize>>();
        let won = owned_half
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .filter(|n| winning.contains(n))
            .count();
        if won > 0 {
            solution_a += 1 << (won - 1);
        }
        cards.push(won);
    }

    let mut counts = vec![1; cards.len()];
    for (i, won) in cards.into_iter().enumerate() {
        for j in 0..won {
            counts[i + j + 1] += counts[i];
        }
    }
    let solution_b = counts.into_iter().sum();

    Ok((solution_a, solution_b))
}
