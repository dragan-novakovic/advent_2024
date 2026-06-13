use crate::advent2016::day1::task1::Direction::Right;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub type Coords = (Option<i32>, Option<i32>);
pub type Move = (Direction, i32);

pub fn runner() {
    let input = "L4, L3, R1, L4, R2, R2, L1, L2, R1, R1, L3, R5, L2, R5, L4, L3, R2, R2, L5, L1, R4, L1, R3, L3, R5, R2, L5, R2, R1, R1, L5, R1, L3, L2, L5, R4, R4, L2, L1, L1, R1, R1, L185, R4, L1, L1, R5, R1, L1, L3, L2, L1, R2, R2, R2, L1, L1, R4, R5, R53, L1, R1, R78, R3, R4, L1, R5, L1, L4, R3, R3, L3, L3, R191, R4, R1, L4, L1, R3, L1, L2, R3, R2, R4, R5, R5, L3, L5, R2, R3, L1, L1, L3, R1, R4, R1, R3, R4, R4, R4, R5, R2, L5, R1, R2, R5, L3, L4, R1, L5, R1, L4, L3, R5, R5, L3, L4, L4, R2, R2, L5, R3, R1, R2, R5, L5, L3, R4, L5, R5, L3, R1, L1, R4, R4, L3, R2, R5, R1, R2, L1, R4, R1, L3, L3, L5, R2, R5, L1, L4, R3, R3, L3, R2, L5, R1, R3, L3, R2, L1, R4, R3, L4, R5, L2, L2, R5, R1, R2, L4, L4, L5, R3, L4";
    let my_cords: Coords = (None, None);
    let target: Coords = (None, None);

    let mvs = convert_input_to_directions(input);
    do_moving(mvs);
}

pub fn do_moving(mvs: Vec<Move>) {
    //update my cords + estimates
    //
}

pub fn convert_input_to_directions(list: &str) -> Vec<Move> {
    list.split(',').fold(vec![], |mut acc, x| {
        let base = x.trim();
        let direction = match &base[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let steps = base[1..].parse::<i32>().unwrap();
        acc.push((direction, steps));
        acc
    })
}

pub fn get_block_distance() -> i32 {
    2
}
