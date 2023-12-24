use anyhow::Result;

use crate::common;
use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

#[allow(clippy::similar_names)]
pub fn main() -> Result<(i64, i64)> {
    let test_range = 200_000_000_000_000f64..=400_000_000_000_000f64;
    let lines = common::read_lines("inputs/24.txt")?;

    let mut solution_a = 0;
    let mut hailstones: Vec<Hailstone> = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        hailstones.push(
            line.split(" @ ")
                .map(|half| {
                    half.split(", ")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_tuple::<Vec3>()
                        .unwrap()
                        .into()
                })
                .collect_tuple()
                .unwrap(),
        );
    }

    for combination in hailstones.iter().combinations(2) {
        let [a, b] = combination[..] else { panic!() };
        let (a_a, b_a) = get_line(*a);
        let (a_b, b_b) = get_line(*b);

        let inter_x = (b_b - b_a) / (a_a - a_b);
        let inter_y = a_a * inter_x + b_a;
        if test_range.contains(&inter_x)
            && test_range.contains(&inter_y)
            && is_future(*a, inter_x, inter_y)
            && is_future(*b, inter_x, inter_y)
        {
            solution_a += 1;
        }
    }

    let cfg = Config::default();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let start_x = Int::new_const(&ctx, "start_x");
    let start_y = Int::new_const(&ctx, "start_y");
    let start_z = Int::new_const(&ctx, "start_z");
    let start_vx = Int::new_const(&ctx, "start_vx");
    let start_vy = Int::new_const(&ctx, "start_vy");
    let start_vz = Int::new_const(&ctx, "start_vz");

    for (
        i,
        (
            Point { x, y, z },
            Point {
                x: vx,
                y: vy,
                z: vz,
            },
        ),
    ) in hailstones.into_iter().enumerate().take(3)
    {
        let t = Int::new_const(&ctx, format!("t_{i}"));
        solver.assert(&t.gt(&Int::from_u64(&ctx, 0)));
        solver.assert(&(x + vx * &t)._eq(&(&start_x + &start_vx * &t)));
        solver.assert(&(y + vy * &t)._eq(&(&start_y + &start_vy * &t)));
        solver.assert(&(z + vz * &t)._eq(&(&start_z + &start_vz * &t)));
    }

    assert_eq!(solver.check(), SatResult::Sat, "Not satisfiable");
    let model = solver.get_model().unwrap();
    let solution_b = model
        .eval(&(start_x + start_y + start_z), true)
        .unwrap()
        .as_i64()
        .unwrap();

    Ok((solution_a, solution_b))
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<Vec3> for Point {
    fn from((x, y, z): Vec3) -> Point {
        Point { x, y, z }
    }
}

#[allow(clippy::cast_precision_loss)]
fn get_line(a: Hailstone) -> (f64, f64) {
    let acceleration = a.1.y as f64 / a.1.x as f64;
    let offset = a.0.y as f64 - acceleration * a.0.x as f64;
    (acceleration, offset)
}

#[allow(clippy::cast_precision_loss)]
fn is_future(hailstone: Hailstone, x: f64, y: f64) -> bool {
    ((hailstone.1.x < 0 && hailstone.0.x as f64 >= x)
        || (hailstone.1.x >= 0 && hailstone.0.x as f64 <= x))
        && ((hailstone.1.y < 0 && hailstone.0.y as f64 >= y)
            || (hailstone.1.y >= 0 && hailstone.0.y as f64 <= y))
}

type Vec3 = (i64, i64, i64);
type Hailstone = (Point, Point);
