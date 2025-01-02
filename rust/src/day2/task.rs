use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/*
This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
 */

pub fn task_two() {
    let reports = load_txt_file_data().unwrap();

    let safe_reports: i32 = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .collect::<Vec<&Vec<i32>>>()
        .len() as i32;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    // start with 3 elements
    // compare and slide
    let mut is_increasing = false;
    let mut is_decreasing = false;
    let mut is_diff_more_then_two = false;
    let mut starters = report.iter().take(3).collect::<Vec<&i32>>();

    if starters.len() == 3 {
        let mut iter = starters.iter();
        let mut prev = iter.next().unwrap().clone();
        let mut next = iter.next().unwrap().clone();
        let mut next_next = iter.next().unwrap().clone();

        if prev < next && next < next_next {
            is_increasing = true;
        } else if prev > next && next > next_next {
            is_decreasing = true;
        }

        if (prev - next).abs() >= 1 && (prev - next).abs() <= 3 {
            is_diff_more_then_two = true;
        }
    }

    (is_decreasing || is_increasing) && is_diff_more_then_two
}

fn load_txt_file_data() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let path = Path::new("src/day2/input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut reports = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        reports.push(numbers);
    }

    Ok(reports)
}
