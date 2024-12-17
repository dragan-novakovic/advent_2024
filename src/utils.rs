// input get

// https://adventofcode.com/2024/day/1/input

pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
