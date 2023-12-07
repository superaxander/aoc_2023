use anyhow::Result;

use crate::common;

#[allow(clippy::bool_to_int_with_if)]
#[allow(clippy::too_many_lines)]
pub fn main() -> Result<(usize, usize)> {
    let values_a = [ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A', ];
    let values_b = [ 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A', ];
    let lines = common::read_lines("inputs/7.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut hands_a = Vec::new();
    let mut hands_b = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (hand_str, bid) = line.split_once(' ').unwrap();
        let hand = hand_str
            .chars()
            .map(|c| values_a.iter().position(|c2| *c2 == c).unwrap())
            .collect::<Vec<_>>();
        let kind = if hand.iter().all(|n| *n == hand[0]) {
            6
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m).count() == 4)
        {
            5
        } else if hand
            .iter()
            .find(|n| hand.iter().filter(|m| n == m).count() == 3)
            .map(|n| {
                hand.iter()
                    .any(|m| n != m && hand.iter().filter(|o| m == *o).count() == 2)
            })
            == Some(true)
        {
            4
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m).count() == 3)
        {
            3
        } else if hand
            .iter()
            .find(|n| hand.iter().filter(|m| n == m).count() == 2)
            .map(|n| {
                hand.iter()
                    .any(|m| n != m && hand.iter().filter(|o| m == *o).count() == 2)
            })
            == Some(true)
        {
            2
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m).count() == 2)
        {
            1
        } else {
            0
        };
        let bid = bid.parse::<usize>()?;
        hands_a.push((kind, hand, bid));

        let hand = hand_str
            .chars()
            .map(|c| values_b.iter().position(|c2| *c2 == c).unwrap())
            .collect::<Vec<_>>();
        let kind = if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m || **m == 0).count() == 5)
        {
            6
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m || **m == 0).count() == 4)
        {
            5
        } else if hand
            .iter()
            .find(|n| hand.iter().filter(|m| n == m || **m == 0).count() == 3)
            .map(|n| {
                hand.iter()
                    .any(|m| n != m && *m != 0 && hand.iter().filter(|o| m == *o).count() == 2)
            })
            == Some(true)
            || hand
                .iter()
                .find(|n| hand.iter().filter(|m| n == m).count() == 3)
                .map(|n| {
                    hand.iter()
                        .any(|m| n != m && hand.iter().filter(|o| m == *o || **o == 0).count() == 2)
                })
                == Some(true)
        {
            4
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m || **m == 0).count() == 3)
        {
            3
        } else if hand
            .iter()
            .find(|n| hand.iter().filter(|m| n == m || **m == 0).count() == 2)
            .map(|n| {
                hand.iter()
                    .any(|m| n != m && hand.iter().filter(|o| m == *o).count() == 2)
            })
            == Some(true)
            || hand
                .iter()
                .find(|n| hand.iter().filter(|m| n == m).count() == 2)
                .map(|n| {
                    hand.iter()
                        .any(|m| n != m && hand.iter().filter(|o| m == *o || **o == 0).count() == 2)
                })
                == Some(true)
        {
            2
        } else if hand
            .iter()
            .any(|n| hand.iter().filter(|m| n == *m || **m == 0).count() == 2)
        {
            1
        } else {
            0
        };
        hands_b.push((kind, hand, bid));
    }

    hands_a.sort();

    for (i, (_, _, bid)) in hands_a.into_iter().enumerate() {
        solution_a += (i + 1) * bid;
    }

    hands_b.sort();

    for (i, (_, _, bid)) in hands_b.into_iter().enumerate() {
        solution_b += (i + 1) * bid;
    }
    Ok((solution_a, solution_b))
}
