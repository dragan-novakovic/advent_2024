//use crate::utils;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/*
2 lists of numbers
find the smallest number in the first list and second list
add the difference.

 */

pub fn load_txt_file_data() -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let path = Path::new("src/day1/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut data_left = Vec::new();
    let mut data_right = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split_whitespace().fold(Vec::new(), |mut acc, x| {
            let result = x.parse::<i32>();
            if let Ok(num) = result {
                acc.push(num);
            }

            acc
        });

        if numbers.len() == 2 {
            data_left.push(numbers[0]);
            data_right.push(numbers[1]);
        }
    }

    Ok((data_left, data_right))
}

pub async fn task_one() -> Result<i32, i32> {
    let (mut left_list, mut right_list) = load_txt_file_data().unwrap();
    left_list.sort();
    right_list.sort();

    let mut max_distance = Vec::<i32>::new();
    let mut left_iter = left_list.iter();
    let mut right_iter = right_list.iter();

    // loop {
    //     let left = left_iter.next();
    //     let right = right_iter.next();

    //     match (left, right) {
    //         (Some(left), Some(right)) => {
    //             if left < right {
    //                 max_distance.push(right - left);
    //             } else {
    //                 max_distance.push(left - right);
    //             }
    //         }
    //         (Some(left), None) => {
    //             println!("Left: {}", left);
    //         }
    //         (None, Some(right)) => {
    //             println!("Right: {}", right);
    //         }
    //         (None, None) => {
    //             break;
    //         }
    //     }
    // }

    // horrible perf, optimize later
    let sim_score = left_iter.fold(0, |mut acc, x| {
        dbg!(acc);
        for i in 0..right_list.len() - 1 {
            dbg!(acc);
            if x.clone() == right_list[i] {
                acc = acc + x;
            }
        }
        acc
    });

    dbg!(sim_score);

    // let max = max_distance.iter().sum::<i32>();

    // dbg!(max);
    Ok(3)
}
