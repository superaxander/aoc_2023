use anyhow::Result;

use crate::common;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/15.txt")?;

    let mut solution_a = 0;
    let mut boxes = vec![Vec::new(); 256];

    for line in lines {
        let line = line?;
        let line = line.trim();

        'string_loop: for string in line.split(',') {
            solution_a += hash(string);

            if string.ends_with('-') {
                let label = &string[0..string.len() - 1];
                let hash = hash(label);
                boxes[hash].retain(|(l, _)| *l != label);
            } else {
                let (label, length) = string.split_once('=').unwrap();
                let hash = hash(label);
                let length = length.parse::<usize>()?;

                for lens in &mut boxes[hash] {
                    if lens.0 == label {
                        lens.1 = length;
                        continue 'string_loop;
                    }
                }
                boxes[hash].push((label.to_owned(), length));
            }
        }
    }

    let mut solution_b = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, (_, k)) in b.into_iter().enumerate() {
            solution_b += (1 + i) * (j + 1) * k;
        }
    }

    Ok((solution_a, solution_b))
}

fn hash(string: &str) -> usize {
    let mut hash = 0;
    for c in string.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}
