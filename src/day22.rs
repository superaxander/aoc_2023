use anyhow::Result;
use itertools::Itertools;

use crate::common;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/22.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut bricks = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let end = end
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        bricks.push(Brick::new(start, end, Vec::new(), Vec::new()));
    }

    bricks.sort_by_key(|b| b.start.2);

    for i in 0..bricks.len() {
        while bricks[i].start.2 != 1 {
            let new_s = down(&bricks[i].start);
            let new_e = down(&bricks[i].end);
            for j in 0..i {
                if bricks[j].collides(new_s, new_e) {
                    bricks[i].needs.push(j);
                    bricks[j].supports.push(i);
                }
            }
            if !bricks[i].needs.is_empty() {
                break;
            }
            bricks[i].start = new_s;
            bricks[i].end = new_e;
        }
    }

    for i in 0..bricks.len() {
        if bricks[i]
            .supports
            .iter()
            .all(|j| bricks[*j].needs.len() > 1)
        {
            solution_a += 1;
        } else {
            solution_b += brittleness(&bricks, i);
        }
    }

    Ok((solution_a, solution_b))
}

fn brittleness(bricks: &[Brick], index: usize) -> usize {
    let mut removed = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(index);
    removed.insert(index);
    while let Some(i) = queue.pop_front() {
        for j in &bricks[i].supports {
            if bricks[*j].needs.iter().all(|k| removed.contains(k)) {
                removed.insert(*j);
                queue.push_back(*j);
            }
        }
    }
    removed.len() - 1
}

type Point = (usize, usize, usize);

fn down(p: &Point) -> Point {
    (p.0, p.1, p.2 - 1)
}

#[derive(Clone, Debug)]
struct Brick {
    start: Point,
    end: Point,
    supports: Vec<usize>,
    needs: Vec<usize>,
}

impl Brick {
    fn new(start: Point, end: Point, supports: Vec<usize>, needs: Vec<usize>) -> Self {
        Brick {
            start,
            end,
            supports,
            needs,
        }
    }

    fn collides(&self, start: Point, end: Point) -> bool {
        self.start.0 <= end.0
            && self.end.0 >= start.0
            && self.start.1 <= end.1
            && self.end.1 >= start.1
            && self.start.2 <= end.2
            && self.end.2 >= start.2
    }
}
