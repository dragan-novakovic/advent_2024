pub static INTPUT_URL: &str = "https://adventofcode.com/2024/day/1/input";

pub type Input = Vec<i32>;

pub fn task_one(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}
