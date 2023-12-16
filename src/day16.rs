use anyhow::Result;

use crate::common;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(clippy::match_on_vec_items)]
pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let height = grid.len();
    let width = grid[0].len();
    let solution_a = count_energized(&grid, (0, 0, Direction::East), width, height);

    let mut solution_b = solution_a;
    for x in 0..width {
        solution_b = solution_b.max(count_energized(
            &grid,
            (x, 0, Direction::South),
            width,
            height,
        ));
        solution_b = solution_b.max(count_energized(
            &grid,
            (x, height - 1, Direction::North),
            width,
            height,
        ));
    }
    for y in 0..height {
        if y != 0 {
            solution_b = solution_b.max(count_energized(
                &grid,
                (0, y, Direction::East),
                width,
                height,
            ));
        }
        solution_b = solution_b.max(count_energized(
            &grid,
            (width - 1, y, Direction::West),
            width,
            height,
        ));
    }

    Ok((solution_a, solution_b))
}

fn move_beam(
    beams: &mut VecDeque<(usize, usize, Direction)>,
    x: usize,
    y: usize,
    d: Direction,
    width: usize,
    height: usize,
) {
    match d {
        Direction::North if y > 0 => beams.push_back((x, y - 1, Direction::North)),
        Direction::South if y < height - 1 => beams.push_back((x, y + 1, Direction::South)),
        Direction::East if x < width - 1 => beams.push_back((x + 1, y, Direction::East)),
        Direction::West if x > 0 => beams.push_back((x - 1, y, Direction::West)),
        _ => {}
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn count_energized(
    grid: &[Vec<char>],
    start: (usize, usize, Direction),
    width: usize,
    height: usize,
) -> usize {
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();
    let mut beams = VecDeque::new();
    beams.push_back(start);

    while let Some((x, y, d)) = beams.pop_front() {
        energized.insert((x, y));

        if !visited.insert((x, y, d)) {
            continue;
        }

        match grid[y][x] {
            '/' => move_beam(
                &mut beams,
                x,
                y,
                match d {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                },
                width,
                height,
            ),
            '\\' => move_beam(
                &mut beams,
                x,
                y,
                match d {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                },
                width,
                height,
            ),
            '|' => match d {
                Direction::North | Direction::South => {
                    move_beam(&mut beams, x, y, d, width, height)
                }
                Direction::East | Direction::West => {
                    move_beam(&mut beams, x, y, Direction::North, width, height);
                    move_beam(&mut beams, x, y, Direction::South, width, height);
                }
            },
            '-' => match d {
                Direction::North | Direction::South => {
                    move_beam(&mut beams, x, y, Direction::East, width, height);
                    move_beam(&mut beams, x, y, Direction::West, width, height);
                }
                Direction::East | Direction::West => move_beam(&mut beams, x, y, d, width, height),
            },
            _ => move_beam(&mut beams, x, y, d, width, height),
        }
    }
    energized.len()
}
