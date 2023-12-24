use anyhow::Result;

use crate::common;
use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
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

    solver.check();
    let model = solver.get_model().expect("Not satisfiable");
    let solution_b = model
        .eval(&(start_x + start_y + start_z), true)
        .unwrap()
        .as_i64()
        .unwrap();

    /*
    let mut points = Vec::new();
    for combination in hailstones.windows(3) {
        let [(A,B),(C,D),(E,F)] = combination[..] else { panic!() };
        let ec = E.sub(&C);
        if ec.dot(&B.cross(&D)) != 0 {
            let s = ec.mul_const(-1).dot(&A.sub(&C).cross(&D))/ec.dot(&B.cross(&D));
            let pos = A.add(&B.mul_const(s));
            if s > 0 {
            points.push((s,pos));
            }
        }
    }
    points.sort();
    for (s,pos) in points {
        println!("{s}, {} {} {}", pos.x, pos.y, pos.z);
    }
    */
    // let A = Point::new(19, 13, 30);
    // let B = Point::new(-2,1,-2);
    // let C = Point::new(18, 19, 22);
    // let D = Point::new(-1,-1,-2);
    // let E = Point::new(20, 25, 34);
    // let F = Point::new(-2,-2,-4);

    // let s = 5;
    // let x = D.z*(A.y - C.y) + D.y*(A.z - C.z) + s*(D.z*B.y - D.y*B.z);
    // let y = D.x*(A.z - C.z) + D.z*(A.x - C.x) + s*(D.x*B.z - D.z*B.x);
    // let z = D.y*(A.x - C.x) + D.x*(A.y - C.y) + s*(D.y*B.x - D.x*B.y);

    // let pq = A.add(&B.mul_const(s)).sub(&C);
    // let normal = pq.cross(&D);
    // println!("{} {} {}", normal.x, normal.y, normal.z);

    // let ec = E.sub(&C);
    // let res = ec.dot(&normal);
    // println!("{}", res);

    // let a =
    //     ec.mul_const(-1).dot(
    //         &D.zxy().mul(&A.yzx().sub(&C.yzx())).add(
    //         &D.yzx().mul(&A.zxy().sub(&C.zxy()))));
    // let b = ec.dot(&D.zxy().mul(&B.yzx()).sub(&D.yzx().mul(&B.zxy())));

    // println!("{a}/{b}");

    // let a = ec.mul_const(-1).dot(
    //         &A.sub(&C).cross(&D));
    // let b = ec.dot(&B.cross(&D));
    // println!("{a}/{b}");

    Ok((solution_a, solution_b))
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

/*
impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point {
            x,y,z
        }
    }

    fn zxy(&self) -> Point {
        Point {
            x: self.z,
            y: self.x,
            z: self.y
        }
    }

    fn yzx(&self) -> Point {
        Point {
            x: self.y,
            y: self.z,
            z: self.x
        }
    }

    fn cross(&self, other: &Point) -> Point {
        self.yzx().mul(&other.zxy()).sub(&self.zxy().mul(&other.yzx()))
    }

    fn dot(&self, other: &Point) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn mul(&self, other: &Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    fn mul_const(&self, other: i64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
*/

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
