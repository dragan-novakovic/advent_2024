use std::any::Any;

fn _print_any(value: &dyn Any) {
    if let Some(string) = value.downcast_ref::<String>() {
        println!("String value: {}", string);
    } else if let Some(int) = value.downcast_ref::<i32>() {
        println!("Integer value: {}", int);
    } else {
        println!("Unknown type");
    }
}

// trait TaskInput {
//     async fn get_task_input() -> Result<Self, Box<dyn std::error::Error>>
//     where
//         Self: Sized;
// }

pub async fn _get_task_input<T: serde::de::DeserializeOwned>(
    input_url: &str,
) -> Result<(), reqwest::Error> {
    let response = reqwest::get(input_url).await?;
    let data = response.text().await?;
    dbg!(data);

    Ok(())
}
