use day1::task::task_one;

mod day1;
mod day2;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _result = task_one().await;
    Ok(())
}
