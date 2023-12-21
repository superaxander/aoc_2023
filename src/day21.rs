use anyhow::Result;

use crate::common;
use std::collections::HashSet;

#[allow(clippy::cast_possible_wrap)]
pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/21.txt")?;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().collect::<Vec<char>>());
    }

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    let quotient = 26_501_365 / grid.len();
    let remainder = 26_501_365 % height - 1;

    let mut frontier = Vec::new();
    let mut new_frontier = Vec::new();
    frontier.push(
        grid.iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|c| *c == 'S')
                    .map(|x| (x as i64, y as i64))
            })
            .unwrap(),
    );

    let mut solution_a = 0;
    let mut visited = HashSet::new();
    let mut min1 = 0;
    let mut min2 = 0;
    let mut points = Vec::new();
    for i in 0.. {
        for (x, y) in frontier.drain(0..) {
            if is_free(&grid, x - 1, y, width, height) && visited.insert((x - 1, y)) {
                new_frontier.push((x - 1, y));
            }
            if is_free(&grid, x, y - 1, width, height) && visited.insert((x, y - 1)) {
                new_frontier.push((x, y - 1));
            }
            if is_free(&grid, x + 1, y, width, height) && visited.insert((x + 1, y)) {
                new_frontier.push((x + 1, y));
            }
            if is_free(&grid, x, y + 1, width, height) && visited.insert((x, y + 1)) {
                new_frontier.push((x, y + 1));
            }
        }
        std::mem::swap(&mut frontier, &mut new_frontier);
        let value = frontier.len() + min2;
        if i == 63 {
            solution_a = value;
        }
        if (i - remainder) % 131 == 0 {
            points.push(value);
            if points.len() == 3 {
                break;
            }
        }
        min2 = min1;
        min1 = value;
    }
    let solution_b = points[0]
        + quotient * (points[1] - points[0])
        + (points[0] + points[2] - 2 * points[1]) * (quotient * (quotient - 1)) / 2;

    Ok((solution_a, solution_b))
}

#[allow(clippy::cast_possible_truncation)]
fn is_free(grid: &[Vec<char>], x: i64, y: i64, width: i64, height: i64) -> bool {
    let x = x.rem_euclid(width) as usize;
    let y = y.rem_euclid(height) as usize;
    grid[y][x] != '#'
}
