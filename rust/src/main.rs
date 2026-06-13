// use day1::task::task_one;
// use day2::task::task_two;

use crate::advent2016::day1::task1::runner;

mod advent2016;
mod day1;
mod day2;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    runner();
    Ok(())
}
