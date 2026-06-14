use crate::advent2016::day1::task1::Direction::Right;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub enum Orientation {
    Bei,
    Nan,
    Xi,
    Dong,
}

pub type Move = (Direction, i32);
pub type Coord = (i32, i32);

#[derive(Debug)]
pub struct Position {
    bei: i32,
    nan: i32,
    xi: i32,
    dong: i32,
    coord_history: Vec<Coord>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            bei: 0,
            nan: 0,
            xi: 0,
            dong: 0,
            coord_history: vec![],
        }
    }
}

pub fn runner() {
    let input = "L4, L3, R1, L4, R2, R2, L1, L2, R1, R1, L3, R5, L2, R5, L4, L3, R2, R2, L5, L1, R4, L1, R3, L3, R5, R2, L5, R2, R1, R1, L5, R1, L3, L2, L5, R4, R4, L2, L1, L1, R1, R1, L185, R4, L1, L1, R5, R1, L1, L3, L2, L1, R2, R2, R2, L1, L1, R4, R5, R53, L1, R1, R78, R3, R4, L1, R5, L1, L4, R3, R3, L3, L3, R191, R4, R1, L4, L1, R3, L1, L2, R3, R2, R4, R5, R5, L3, L5, R2, R3, L1, L1, L3, R1, R4, R1, R3, R4, R4, R4, R5, R2, L5, R1, R2, R5, L3, L4, R1, L5, R1, L4, L3, R5, R5, L3, L4, L4, R2, R2, L5, R3, R1, R2, R5, L5, L3, R4, L5, R5, L3, R1, L1, R4, R4, L3, R2, R5, R1, R2, L1, R4, R1, L3, L3, L5, R2, R5, L1, L4, R3, R3, L3, R2, L5, R1, R3, L3, R2, L1, R4, R3, L4, R5, L2, L2, R5, R1, R2, L4, L4, L5, R3, L4";
    let mut position: Position = Position {
        ..Position::default()
    };
    let mvs = convert_input_to_directions(input);
    let result = do_moving(position, mvs);
    dbg!(result);
}

pub fn do_moving(mut pos: Position, mvs: Vec<Move>) -> i32 {
    let mut current_orientation = Orientation::Bei;
    //update my cords + estimates
    for (dir, steps) in mvs {
        match dir {
            Direction::Left => match current_orientation {
                Orientation::Bei => current_orientation = Orientation::Xi,
                Orientation::Nan => current_orientation = Orientation::Dong,
                Orientation::Xi => current_orientation = Orientation::Nan,
                Orientation::Dong => current_orientation = Orientation::Bei,
            },

            Direction::Right => match current_orientation {
                Orientation::Bei => current_orientation = Orientation::Dong,
                Orientation::Nan => current_orientation = Orientation::Xi,
                Orientation::Xi => current_orientation = Orientation::Bei,
                Orientation::Dong => current_orientation = Orientation::Nan,
            },
        }

        match current_orientation {
            Orientation::Bei => pos.bei += steps,
            Orientation::Nan => pos.nan += steps,
            Orientation::Xi => pos.xi += steps,
            Orientation::Dong => pos.dong += steps,
        }
    }

    (pos.bei - pos.nan).abs() + (pos.dong - pos.xi).abs()
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

/*
* Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny HQ is actually at the first location you visit twice.

For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4 blocks away, due East.

How many blocks away is the first location you visit twice?
*
*
*/
