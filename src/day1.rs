use anyhow::Result;

use crate::common;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/1.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        let nums = line
            .chars()
            .filter(char::is_ascii_digit)
            .map(|c| c as i64 - '0' as i64)
            .collect::<Vec<i64>>();
        solution_a += nums[0] * 10 + nums.last().unwrap();

        let mut digits: Vec<i64> = Vec::new();
        let chars = line.chars().collect::<Vec<char>>();
        for (i, c) in chars.iter().copied().enumerate() {
            match c {
                '0'..='9' => {
                    digits.push(c as i64 - '0' as i64);
                }
                'e' => {
                    if i > 1 && chars[i - 2..i] == ['o', 'n'] {
                        digits.push(1);
                    } else if i > 2 && chars[i - 3..i] == ['n', 'i', 'n'] {
                        digits.push(9);
                    } else if i > 3 && chars[i - 4..i] == ['t', 'h', 'r', 'e'] {
                        digits.push(3);
                    } else if i > 2 && chars[i - 3..i] == ['f', 'i', 'v'] {
                        digits.push(5);
                    }
                }

                'o' => {
                    if i > 1 && chars[i - 2..i] == ['t', 'w'] {
                        digits.push(2);
                    }
                }
                'r' => {
                    if i > 2 && chars[i - 3..i] == ['f', 'o', 'u'] {
                        digits.push(4);
                    }
                }
                'x' => {
                    if i > 1 && chars[i - 2..i] == ['s', 'i'] {
                        digits.push(6);
                    }
                }
                'n' => {
                    if i > 3 && chars[i - 4..i] == ['s', 'e', 'v', 'e'] {
                        digits.push(7);
                    }
                }
                't' => {
                    if i > 3 && chars[i - 4..i] == ['e', 'i', 'g', 'h'] {
                        digits.push(8);
                    }
                }
                _ => {}
            }
        }
        solution_b += digits[0] * 10 + digits.last().unwrap();
    }

    Ok((solution_a, solution_b))
}
