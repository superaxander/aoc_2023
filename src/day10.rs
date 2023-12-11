use anyhow::Result;

use crate::common;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(clippy::match_on_vec_items)]
pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/10.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let height = grid.len();
    let width = grid[0].len();

    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|c| *c == 'S').map(|x| (x, y)))
        .unwrap();

    let mut main_loop = HashSet::new();
    main_loop.insert((start_x, start_y));
    let (mut x, mut y, mut direction) = determine_direction(&grid, start_x, start_y, width, height);

    let initial_d = direction;

    for i in 1.. {
        main_loop.insert((x, y));
        match direction {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }

        if grid[y][x] == 'S' {
            solution_a = (i + 1) / 2;
            break;
        }

        direction = get_directions(grid[y][x])
            .and_then(|lr| get_opposite(lr, direction.flip()))
            .unwrap();
    }

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if !main_loop.contains(&(x, y)) {
                *c = '.';
            }
        }
    }

    let (start_x, start_y) = (
        (0..width).find(|x| !main_loop.contains(&(*x, y))).unwrap(),
        0,
    );

    let mut frontier = left_hand_side_rule(
        &grid, &main_loop, start_x, start_y, width, height, initial_d,
    );
    let mut visited = HashSet::new();

    while let Some((x, y)) = frontier.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }
        if !main_loop.contains(&(x, y)) {
            solution_b += 1;
            #[cfg(debug_assertions)]
            {
                grid[y][x] = 'O';
            }
            if x < width - 1 {
                frontier.push_back((x + 1, y));
            }
            if x > 0 {
                frontier.push_back((x - 1, y));
            }
            if y < height - 1 {
                frontier.push_back((x, y + 1));
            }
            if y > 0 {
                frontier.push_back((x, y - 1));
            }
        }
    }

    #[cfg(debug_assertions)]
    for row in &grid {
        for c in row {
            if *c == 'O' {
                print!("\x1b[1;31mO\x1b[0m");
            } else if *c == '.' {
                print!("\x1b[1;32mI\x1b[0m");
            } else {
                print!("{c}");
            }
        }
        println!();
    }

    let solution_b = width * height - solution_b - main_loop.len();

    Ok((solution_a, solution_b))
}

fn determine_direction(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> (usize, usize, Direction) {
    if x > 0
        && let Some(d) =
            get_directions(grid[y][x - 1]).and_then(|lr| get_opposite(lr, Direction::East))
    {
        (x - 1, y, d)
    } else if x < width - 1
        && let Some(d) =
            get_directions(grid[y][x + 1]).and_then(|lr| get_opposite(lr, Direction::West))
    {
        (x + 1, y, d)
    } else if y > 0
        && let Some(d) =
            get_directions(grid[y - 1][x]).and_then(|lr| get_opposite(lr, Direction::South))
    {
        (x, y - 1, d)
    } else if y < height - 1
        && let Some(d) =
            get_directions(grid[y + 1][x]).and_then(|lr| get_opposite(lr, Direction::North))
    {
        (x, y + 1, d)
    } else {
        panic!("No direction found")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn flip(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

fn get_opposite((left, right): (Direction, Direction), d: Direction) -> Option<Direction> {
    if left == d {
        Some(right)
    } else if right == d {
        Some(left)
    } else {
        None
    }
}

fn get_directions(c: char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::North, Direction::South)),
        '-' => Some((Direction::East, Direction::West)),
        'L' => Some((Direction::North, Direction::East)),
        'J' => Some((Direction::North, Direction::West)),
        '7' => Some((Direction::South, Direction::West)),
        'F' => Some((Direction::South, Direction::East)),
        _ => None,
    }
}

fn get_outside_directions(c: char, direction: Direction, left: bool) -> Option<[Direction; 2]> {
    if left {
        match c {
            'L' if direction == Direction::North => Some([Direction::West, Direction::South]),
            'J' if direction == Direction::West => Some([Direction::East, Direction::South]),
            '7' if direction == Direction::South => Some([Direction::East, Direction::North]),
            'F' if direction == Direction::East => Some([Direction::West, Direction::North]),
            '|' | '-' => Some([direction.left(), direction.left()]),

            _ => None,
        }
    } else {
        match c {
            'L' if direction == Direction::East => Some([Direction::West, Direction::South]),
            'J' if direction == Direction::North => Some([Direction::East, Direction::South]),
            '7' if direction == Direction::West => Some([Direction::East, Direction::North]),
            'F' if direction == Direction::South => Some([Direction::West, Direction::North]),
            '|' | '-' => Some([direction.right(), direction.right()]),
            _ => None,
        }
    }
}

fn left_hand_side_rule(
    grid: &[Vec<char>],
    main_loop: &HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
    initial_d: Direction,
) -> VecDeque<(usize, usize)> {
    let (main_x, main_y, outside_direction) =
        find_main_loop(main_loop, start_x, start_y, width, height);
    let (mut x, mut y, mut direction) = determine_direction(grid, main_x, main_y, width, height);

    let left = direction.left() != outside_direction;
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();

    loop {
        if let Some(outside_directions) = get_outside_directions(grid[y][x], direction, left) {
            for outside_direction in outside_directions {
                match outside_direction {
                    Direction::South if y < height - 1 => frontier.push_back((x, y + 1)),
                    Direction::North if y > 0 => frontier.push_back((x, y - 1)),
                    Direction::East if x < width - 1 => frontier.push_back((x + 1, y)),
                    Direction::West if x > 0 => frontier.push_back((x - 1, y)),
                    _ => {}
                }
            }
        }
        match direction {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }

        if x == main_x && y == main_y {
            break;
        }

        if grid[y][x] == 'S' {
            direction = initial_d;
        } else {
            direction = get_directions(grid[y][x])
                .and_then(|lr| get_opposite(lr, direction.flip()))
                .unwrap();
        }
    }

    frontier
}

fn find_main_loop(
    main_loop: &HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> (usize, usize, Direction) {
    let mut visited = HashSet::new();
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back((start_x, start_y));

    while let Some((x, y)) = frontier.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }
        if !main_loop.contains(&(x, y)) {
            if x < width - 1 {
                if main_loop.contains(&(x + 1, y)) {
                    return (x + 1, y, Direction::West);
                }
                frontier.push_back((x + 1, y));
            }
            if x > 0 {
                if main_loop.contains(&(x - 1, y)) {
                    return (x - 1, y, Direction::East);
                }
                frontier.push_back((x - 1, y));
            }
            if y < height - 1 {
                if main_loop.contains(&(x, y + 1)) {
                    return (x, y + 1, Direction::North);
                }
                frontier.push_back((x, y + 1));
            }
            if y > 0 {
                if main_loop.contains(&(x, y - 1)) {
                    return (x, y - 1, Direction::South);
                }
                frontier.push_back((x, y - 1));
            }
        }
    }

    panic!("We didn't find the main loop")
}
