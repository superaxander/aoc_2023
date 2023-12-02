use anyhow::Result;

use crate::common;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/2.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        let (first, second) = line.split_once(": ").unwrap();
        let id = first.split_once(' ').unwrap().1.parse::<usize>()?;

        let mut sets = Vec::new();
        let mut valid_game = true;
        for set in second.split("; ") {
            let mut bag = HashMap::new();

            for draw in set.split(", ") {
                let (num, colour) = draw.split_once(' ').unwrap();
                let num = num.parse::<usize>()?;
                match colour {
                    "red" => {
                        if num > 12 {
                            valid_game = false;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            valid_game = false;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            valid_game = false;
                        }
                    }
                    _ => {}
                }
                bag.insert(colour, num);
            }

            sets.push(bag);
        }

        if valid_game {
            solution_a += id;
        }

        let red = sets
            .iter()
            .map(|bag| *bag.get("red").unwrap_or(&0))
            .max()
            .unwrap();
        let green = sets
            .iter()
            .map(|bag| *bag.get("green").unwrap_or(&0))
            .max()
            .unwrap();
        let blue = sets
            .iter()
            .map(|bag| *bag.get("blue").unwrap_or(&0))
            .max()
            .unwrap();
        solution_b += red * green * blue;
    }

    Ok((solution_a, solution_b))
}
