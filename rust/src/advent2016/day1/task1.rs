use crate::advent2016::day1::task1::Direction::Right;

pub enum Direction {
    Left,
    Right,
}

pub type Coords<i32, i32>;
pub type Move<Direction, i32>;

pub fn runner() {
    let my_cords: Coords<i32, i32> = (None, None);

    let first_mvs = vec![(Direction::Right, 2), (Direction::Left, 3)];
    let second_mvs = vec![
        (Direction::Right, 2),
        (Direction::Right, 2),
        (Direction::Right, 2),
    ];

    do_moving(first_move);
}

pub fn do_moving(mvs: Vec<Move>) {
    //update my cords + estimates
}

pub fn get_block_distance() -> i32 {
    2
}
