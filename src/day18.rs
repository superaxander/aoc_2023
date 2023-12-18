use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/18.txt")?;

    let mut moves_a = Vec::new();
    let mut moves_b = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let (direction, remaining) = line.split_once(' ').unwrap();
        let (amount, remaining) = remaining.split_once(' ').unwrap();
        let direction = match direction {
            "L" => Direction::West,
            "R" => Direction::East,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => panic!("Unknown direction {direction}"),
        };
        let amount = amount.parse::<i64>()?;
        moves_a.push((direction, amount));
        let hex = i64::from_str_radix(&remaining[2..remaining.len() - 1], 16)?;
        let direction = hex & 0b1111;
        let direction = match direction {
            2 => Direction::West,
            0 => Direction::East,
            3 => Direction::North,
            1 => Direction::South,
            _ => panic!("Unknown direction {direction}"),
        };
        let amount = hex >> 4;
        moves_b.push((direction, amount));
    }

    let solution_a = measure_polygon(&moves_a);
    let solution_b = measure_polygon(&moves_b);

    Ok((solution_a, solution_b))
}

fn measure_polygon(moves: &[(Direction, i64)]) -> i64 {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut boundary = 0;

    for (direction, amount) in moves {
        boundary += *amount;
        points.push((x, y));
        match direction {
            Direction::North => y -= *amount,
            Direction::South => y += *amount,
            Direction::East => x += *amount,
            Direction::West => x -= *amount,
        }
    }
    points.push((x, y));
    points.push((0, 0));

    let area: i64 = points
        .into_iter()
        .map_windows(|[(x1, y1), (x2, y2)]| (y1 + y2) * (x1 - x2))
        .sum();
    (area / 2).abs() + (boundary + 2) / 2
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
