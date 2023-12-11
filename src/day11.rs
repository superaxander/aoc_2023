use anyhow::Result;

use crate::common;
use itertools::Itertools;

pub fn main(do_b: bool) -> Result<usize> {
    let lines = common::read_lines("inputs/11.txt")?;

    let mut galaxies = Vec::new();
    let mut height = 0;
    let mut width = 0;

    for (y, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                height = height.max(y);
                width = width.max(x);
            }
        }
    }

    let mut i = 0;
    while i < width {
        if !galaxies.iter().any(|(x, _)| *x == i) {
            for (x, _) in &mut galaxies {
                if *x >= i {
                    if do_b {
                        *x += 999_999;
                    } else {
                        *x += 1;
                    }
                }
            }
            if do_b {
                i += 999_999;
                width += 999_999;
            } else {
                i += 1;
                width += 1;
            }
        }
        i += 1;
    }

    let mut i = 0;
    while i < height {
        if !galaxies.iter().any(|(_, y)| *y == i) {
            for (_, y) in &mut galaxies {
                if *y >= i {
                    if do_b {
                        *y += 999_999;
                    } else {
                        *y += 1;
                    }
                }
            }
            if do_b {
                i += 999_999;
                height += 999_999;
            } else {
                i += 1;
                height += 1;
            }
        }
        i += 1;
    }

    let mut solution = 0;
    for vec in galaxies.into_iter().combinations(2) {
        if let [(ax, ay), (bx, by)] = &vec[..] {
            solution += ax.abs_diff(*bx) + ay.abs_diff(*by);
        }
    }

    Ok(solution)
}
