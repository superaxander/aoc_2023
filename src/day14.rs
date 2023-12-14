use anyhow::Result;

use crate::common;
use std::collections::HashMap;


pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/14.txt")?;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut occurrences = HashMap::new();
    occurrences.insert(grid.clone(), 0);
    let height = grid.len();
    let width = grid[0].len();

    tilt_north(&mut grid, width, height);
    let solution_a = grid
        .iter()
        .enumerate()
        .map(|(y, row)| (height - y) * row.iter().filter(|c| **c == 'O').count())
        .sum();
    tilt_west(&mut grid, width, height);
    tilt_south(&mut grid, width, height);
    tilt_east(&mut grid, width, height);

    occurrences.insert(grid.clone(), 1);

    let mut i = 2;
    while i <= 1_000_000_000 {
        let _ = tilt_north(&mut grid, width, height);
        tilt_west(&mut grid, width, height);
        tilt_south(&mut grid, width, height);
        tilt_east(&mut grid, width, height);
        if let Some(j) = occurrences.insert(grid.clone(), i) {
            let cycle = i - j;
            let remaining = (1_000_000_000 - i) % cycle;
            i = 1_000_000_000 - remaining;
        }
        i += 1;
    }

    let solution_b = grid
        .iter()
        .enumerate()
        .map(|(y, row)| (height - y) * row.iter().filter(|c| **c == 'O').count())
        .sum();

    Ok((solution_a, solution_b))
}

#[allow(clippy::needless_range_loop)]
fn tilt_north(grid: &mut [Vec<char>], width: usize, height: usize) -> usize {
    let mut load = 0;
    for y in 0..height {
        'x_loop: for x in 0..width {
            if grid[y][x] == 'O' {
                grid[y][x] = '.';
                for cy in (0..y).rev() {
                    if grid[cy][x] != '.' {
                        grid[cy + 1][x] = 'O';
                        load += height - cy - 1;
                        continue 'x_loop;
                    }
                }
                grid[0][x] = 'O';
                load += height;
            }
        }
    }
    load
}

#[allow(clippy::needless_range_loop)]
fn tilt_west(grid: &mut [Vec<char>], width: usize, height: usize) {
    for x in 0..width {
        'y_loop: for y in 0..height {
            if grid[y][x] == 'O' {
                grid[y][x] = '.';
                for cx in (0..x).rev() {
                    if grid[y][cx] != '.' {
                        grid[y][cx + 1] = 'O';
                        continue 'y_loop;
                    }
                }
                grid[y][0] = 'O';
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn tilt_south(grid: &mut [Vec<char>], width: usize, height: usize) {
    for y in (0..height).rev() {
        'x_loop: for x in 0..width {
            if grid[y][x] == 'O' {
                grid[y][x] = '.';
                for cy in (y + 1)..height {
                    if grid[cy][x] != '.' {
                        grid[cy - 1][x] = 'O';
                        continue 'x_loop;
                    }
                }
                grid[height - 1][x] = 'O';
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn tilt_east(grid: &mut [Vec<char>], width: usize, height: usize) {
    for x in (0..width).rev() {
        'y_loop: for y in 0..height {
            if grid[y][x] == 'O' {
                grid[y][x] = '.';
                for cx in (x + 1)..width {
                    if grid[y][cx] != '.' {
                        grid[y][cx - 1] = 'O';
                        continue 'y_loop;
                    }
                }
                grid[y][width - 1] = 'O';
            }
        }
    }
}
