use anyhow::Result;

use crate::common;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/17.txt")?;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        );
    }

    let height = grid.len();
    let width = grid[0].len();

    let solution_a = calculate_shortest_path(&grid, width, height, 0, 3);
    let solution_b = calculate_shortest_path(&grid, width, height, 4, 10);

    Ok((solution_a, solution_b))
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct State(usize, usize, Direction, usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

fn calculate_shortest_path(
    grid: &[Vec<usize>],
    width: usize,
    height: usize,
    min_straight: usize,
    max_straight: usize,
) -> usize {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, State(0, 0, Direction::East, 0))));
    let mut g_scores = HashMap::new();

    while let Some(Reverse((score, state))) = frontier.pop() {
        let State(x, y, direction, repeating) = state.clone();
        if x == width - 1 && y == height - 1 {
            return score;
        }

        if repeating >= min_straight {
            for d in [direction.left(), direction.right()] {
                if let Some((x, y)) = match d {
                    Direction::North if y > 0 => Some((x, y - 1)),
                    Direction::South if y < height - 1 => Some((x, y + 1)),
                    Direction::East if x < width - 1 => Some((x + 1, y)),
                    Direction::West if x > 0 => Some((x - 1, y)),
                    _ => None,
                } {
                    let new_score = score + grid[y][x];
                    let new_state = State(x, y, d, 1);
                    if *g_scores.get(&new_state).unwrap_or(&usize::MAX) > new_score {
                        g_scores.insert(new_state.clone(), new_score);
                        frontier.push(Reverse((new_score, new_state)));
                    }
                }
            }
        }

        if repeating < max_straight {
            if let Some((x, y)) = match direction {
                Direction::North if y > 0 => Some((x, y - 1)),
                Direction::South if y < height - 1 => Some((x, y + 1)),
                Direction::East if x < width - 1 => Some((x + 1, y)),
                Direction::West if x > 0 => Some((x - 1, y)),
                _ => None,
            } {
                let new_score = score + grid[y][x];
                let new_state = State(x, y, direction, repeating + 1);
                if *g_scores.get(&new_state).unwrap_or(&usize::MAX) > new_score {
                    g_scores.insert(new_state.clone(), new_score);
                    frontier.push(Reverse((new_score, new_state)));
                }
            }
        }
    }

    usize::MAX
}
