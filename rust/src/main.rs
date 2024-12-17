mod day1;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..1 {
        let input = utils::get_task_input::<day1::task1::Input>(day1::task1::INTPUT_URL).await?;
    }
    Ok(())
}
