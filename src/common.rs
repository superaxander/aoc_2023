use anyhow::Result;
use core::convert::AsRef;
use core::result::Result::Ok;
use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;
use std::time::Instant;

pub static RE_WS: LazyLock<Regex> = LazyLock::new(|| Regex::new("\\s+").unwrap());

pub fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[allow(dead_code)]
pub enum Day<SolutionA: Display, SolutionB: Display> {
    Combined(fn() -> Result<(SolutionA, SolutionB)>),
    Separated(fn() -> Result<SolutionA>, fn() -> Result<SolutionB>),
}

impl<SolutionA: Display, SolutionB: Display> Day<SolutionA, SolutionB> {
    fn run_with_result(&self, name: &str) -> Result<()> {
        match self {
            Day::Combined(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::Separated(func_a, func_b) => {
                let now = Instant::now();
                let solution_a = func_a()?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let solution_b = func_b()?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
        }
        Ok(())
    }
}

pub(crate) trait Runnable {
    fn run(&self, name: &str);
}

impl<SolutionA: Display, SolutionB: Display> Runnable for Day<SolutionA, SolutionB> {
    fn run(&self, name: &str) {
        if let Err(e) = self.run_with_result(name) {
            error!("Error occurred running {}: {}", name, e);
        }
    }
}
