use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/13.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            let (a, b) = calculate_solution(&mut grid);
            solution_a += a;
            solution_b += b;
            grid.clear();
        } else {
            grid.push(line.chars().collect::<Vec<char>>());
        }
    }
    if !grid.is_empty() {
        let (a, b) = calculate_solution(&mut grid);
        solution_a += a;
        solution_b += b;
    }

    Ok((solution_a, solution_b))
}

fn calculate_solution(grid: &mut [Vec<char>]) -> (usize, usize) {
    let (left, above) = get_score(grid, 0, 0);
    let score = left + 100 * above;
    let solution_a = score;

    let mut solution_b = 0;
    let candidates = find_smudge_candidates(grid);
    for (x, y) in candidates {
        if grid[y][x] == '.' {
            grid[y][x] = '#';
        } else {
            grid[y][x] = '.';
        }
        let (left, above) = get_score(grid, left, above);
        let new_score = left + 100 * above;
        if new_score > 0 && new_score != score {
            solution_b = new_score;
            break;
        }
        if grid[y][x] == '.' {
            grid[y][x] = '#';
        } else {
            grid[y][x] = '.';
        }
    }
    (solution_a, solution_b)
}

fn get_score(grid: &[Vec<char>], old_left: usize, old_above: usize) -> (usize, usize) {
    let height = grid.len();
    let width = grid[0].len();
    let mut rows_above = 0;
    let mut rows_left = 0;
    for y in 1..height {
        let mut found = false;
        for i in 0..y {
            if y + y - i - 1 < height && grid[i] != grid[y + y - i - 1] {
                found = true;
                break;
            }
        }
        if !found && y != old_above {
            rows_above = y;
            break;
        }
    }

    for x in 1..width {
        let mut found = false;
        for i in 0..x {
            if x + x - i - 1 < width && grid.iter().any(|row| row[i] != row[x + x - i - 1]) {
                found = true;
                break;
            }
        }
        if !found && x != old_left {
            rows_left = x;
            break;
        }
    }
    (rows_left, rows_above)
}

fn find_smudge_candidates(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    let height = grid.len();
    let width = grid[0].len();
    for y in 1..height {
        for i in 0..y {
            if y + y - i - 1 < height {
                let differences = grid[i]
                    .iter()
                    .enumerate()
                    .filter(|(j, c)| **c != grid[y + y - i - 1][*j])
                    .map(|(j, _)| j)
                    .collect::<Vec<usize>>();
                if differences.len() == 1 {
                    candidates.push((differences[0], y));
                    candidates.push((differences[0], y + y - i - 1));
                }
            }
        }
    }

    for x in 1..width {
        for i in 0..x {
            if x + x - i - 1 < width {
                let differences = grid
                    .iter()
                    .enumerate()
                    .filter(|(_, row)| row[i] != row[x + x - i - 1])
                    .map(|(y, _)| y)
                    .collect::<Vec<usize>>();
                if differences.len() == 1 {
                    candidates.push((x, differences[0]));
                    candidates.push((x + x - i - 1, differences[0]));
                }
            }
        }
    }
    candidates
}
