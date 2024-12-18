use std::any::Any;

fn print_any(value: &dyn Any) {
    if let Some(string) = value.downcast_ref::<String>() {
        println!("String value: {}", string);
    } else if let Some(int) = value.downcast_ref::<i32>() {
        println!("Integer value: {}", int);
    } else {
        println!("Unknown type");
    }
}

trait TaskInput {
    async fn get_task_input() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

pub async fn get_task_input<T: serde::de::DeserializeOwned>(
    input_url: &str,
) -> Result<T, reqwest::Error> {
    let resp = reqwest::get(input_url).await?.json::<T>().await?;

    Ok(resp)
}
