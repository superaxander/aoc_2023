use anyhow::Result;

use crate::common;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (usize, usize);

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/23.txt")?;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().collect::<Vec<char>>());
    }

    let goal = (grid[0].len() - 2, grid.len() - 1);
    let solution_a = solve(&grid, goal, true);
    let solution_b = solve(&grid, goal, false);

    Ok((solution_a, solution_b))
}

fn dfs(
    visited: &mut HashSet<Point>,
    neighbours: &HashMap<Point, Vec<(Point, usize)>>,
    current: Point,
    goal: Point,
) -> Option<usize> {
    if current == goal {
        return Some(0);
    }

    let mut largest: Option<usize> = None;
    visited.insert(current);
    for (nb, cost) in &neighbours[&current] {
        if !visited.contains(nb) {
            largest = largest.max(dfs(visited, neighbours, *nb, goal).map(|n| n + *cost));
        }
    }
    visited.remove(&current);

    largest
}

#[allow(clippy::match_on_vec_items)]
fn solve(grid: &[Vec<char>], goal: Point, no_climb: bool) -> usize {
    let mut neighbours: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(((1, 0), 1, (1, 0), (1, 1)));

    while let Some((origin, count, last, (x, y))) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            neighbours.entry(origin).or_default().push(((x, y), count));
            neighbours.entry((x, y)).or_default().push((origin, count));
            continue;
        }
        if goal == (x, y) {
            neighbours.entry(origin).or_default().push(((x, y), count));
            continue;
        }
        let mut nbs = Vec::new();
        match grid[y][x - 1] {
            '#' => {}
            '>' if no_climb => {}
            _ => nbs.push((x - 1, y)),
        }
        match grid[y][x + 1] {
            '#' => {}
            '<' if no_climb => {}
            _ => nbs.push((x + 1, y)),
        }
        match grid[y - 1][x] {
            '#' => {}
            'v' if no_climb => {}
            _ => nbs.push((x, y - 1)),
        }
        match grid[y + 1][x] {
            '#' => {}
            '^' if no_climb => {}
            _ => nbs.push((x, y + 1)),
        }
        if nbs.len() < 3 {
            for nb in nbs {
                if nb != last {
                    queue.push_back((origin, count + 1, (x, y), nb));
                }
            }
        } else {
            visited.insert((x, y));
            neighbours.entry(origin).or_default().push(((x, y), count));
            neighbours.entry((x, y)).or_default().push((origin, count));
            for nb in nbs {
                if nb != last {
                    queue.push_back(((x, y), 1, (x, y), nb));
                }
            }
        }
    }

    for nbs in neighbours.values_mut() {
        nbs.sort_unstable_by(|(nb_a, cost_a), (nb_b, cost_b)| {
            nb_a.cmp(nb_b).then(cost_b.cmp(cost_a))
        });
        nbs.dedup_by_key(|(nb, _)| *nb);
    }

    dfs(&mut HashSet::new(), &neighbours, (1, 0), goal).unwrap()
}
