use crate::solutions::exec_day;
use std::env;

mod grid;
mod intcode;
mod solutions;
mod solver;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    exec_day(day);
}
