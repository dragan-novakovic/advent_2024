use day1::task::task_one;
use day2::task::task_two;

mod day1;
mod day2;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _result = task_two();
    Ok(())
}
