use anyhow::Result;

use crate::common;

use std::collections::HashMap;
use std::collections::HashSet;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/3.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut gears = HashMap::new();

    let mut current = String::new();
    let mut adjacent = false;
    let mut gear_connections = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let c = grid[y][x];
            if c.is_ascii_digit() {
                if !adjacent {
                    if x > 0 {
                        if is_symbol(grid[y][x - 1]) {
                            adjacent = true;
                            if grid[y][x - 1] == '*' {
                                gear_connections.insert((y, x - 1));
                            }
                        }
                        if y > 0 && is_symbol(grid[y - 1][x - 1]) {
                            adjacent = true;
                            if grid[y - 1][x - 1] == '*' {
                                gear_connections.insert((y - 1, x - 1));
                            }
                        }
                        if y < grid.len() - 1 && is_symbol(grid[y + 1][x - 1]) {
                            adjacent = true;
                            if grid[y + 1][x - 1] == '*' {
                                gear_connections.insert((y + 1, x - 1));
                            }
                        }
                    }
                    if x < grid[y].len() - 1 {
                        if is_symbol(grid[y][x + 1]) {
                            adjacent = true;
                            if grid[y][x + 1] == '*' {
                                gear_connections.insert((y, x + 1));
                            }
                        }

                        if y > 0 && is_symbol(grid[y - 1][x + 1]) {
                            adjacent = true;
                            if grid[y - 1][x + 1] == '*' {
                                gear_connections.insert((y - 1, x + 1));
                            }
                        }
                        if y < grid.len() - 1 && is_symbol(grid[y + 1][x + 1]) {
                            adjacent = true;
                            if grid[y + 1][x + 1] == '*' {
                                gear_connections.insert((y + 1, x + 1));
                            }
                        }
                    }

                    if y > 0 && is_symbol(grid[y - 1][x]) {
                        adjacent = true;
                        if grid[y - 1][x] == '*' {
                            gear_connections.insert((y - 1, x));
                        }
                    }

                    if y < grid.len() - 1 && is_symbol(grid[y + 1][x]) {
                        adjacent = true;
                        if grid[y + 1][x] == '*' {
                            gear_connections.insert((y + 1, x));
                        }
                    }
                }
                current.push(c);
            } else if !current.is_empty() {
                if adjacent {
                    let num = current.parse::<usize>()?;
                    solution_a += num;
                    for gear in gear_connections.drain() {
                        gears.entry(gear).or_insert(Vec::new()).push(num);
                    }
                }
                current.clear();
                adjacent = false;
            }
        }
    }

    for (_, nums) in gears {
        if nums.len() == 2 {
            solution_b += nums[0] * nums[1];
        }
    }

    Ok((solution_a, solution_b))
}
