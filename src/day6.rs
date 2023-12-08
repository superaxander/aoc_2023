use anyhow::Result;

use crate::common;
use crate::common::RE_WS;
use std::fmt::Write;

pub fn main() -> Result<(usize, usize)> {
    let mut lines = common::read_lines("inputs/6.txt")?;

    let line = lines.next().unwrap()?;
    let line = line.trim();
    let times = RE_WS
        .split(line)
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let line = lines.next().unwrap()?;
    let line = line.trim();
    let distances = RE_WS
        .split(line)
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut solution_a = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        solution_a *= binary_range_search(1, *time - 1, *time, *distance);
    }

    let time = times
        .into_iter()
        .fold(String::new(), |mut out, n| {
            let _ = write!(out, "{n}");
            out
        })
        .parse::<usize>()?;
    let distance = distances
        .into_iter()
        .fold(String::new(), |mut out, n| {
            let _ = write!(out, "{n}");
            out
        })
        .parse::<usize>()?;
    let solution_b = binary_range_search(1, time - 1, time, distance);

    Ok((solution_a, solution_b))
}

fn binary_range_search(mut low: usize, mut high: usize, time: usize, distance: usize) -> usize {
    while high >= low {
        let mid = (high + low) / 2;
        if mid * (time - mid) > distance {
            return match (
                low * (time - low) > distance,
                high * (time - high) > distance,
            ) {
                (true, true) => high - low + 1,
                (true, false) => {
                    binary_range_search(mid + 1, high - 1, time, distance) + mid - low + 1
                }
                (false, true) => {
                    binary_range_search(low + 1, mid - 1, time, distance) + high - mid + 1
                }
                (false, false) => {
                    binary_range_search(low + 1, mid - 1, time, distance)
                        + binary_range_search(mid + 1, high - 1, time, distance)
                        + 1
                }
            };
        }
        let left = mid - 1;
        let right = mid + 1;
        if left * (time - left) > right * (time - right) {
            high = left;
        } else {
            low = right;
        }
    }
    0
}
