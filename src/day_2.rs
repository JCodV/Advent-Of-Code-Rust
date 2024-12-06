use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run_day2(path: &str) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let _ = create_reports(path, &mut reports);
    let safe_levels = find_safe_levels(&reports);
    println!("Number of safe levels {}.", safe_levels);
}

pub fn create_reports(path: &str, reports: &mut Vec<Vec<i32>>) -> std::io::Result<()> {
    let file = File::open(path)?;
    // ignore mut error, provided in all examples
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        let mut v = Vec::new();
        for num in line.unwrap().split_whitespace() {
            let num_val: i32 = num.parse().expect("Could not parse num_val");
            v.push(num_val);
        }
        reports.push(v);
    }

    //println!("{:?}", reports);
    Ok(())
}

pub fn is_level_safe(level: &Vec<i32>) -> bool {
    let mut is_safe: bool = true;
    let is_increasing: bool = if level[0] < level[1] { true } else { false };

    if !is_diff_valid(level) {
        is_safe = false;
    }

    if (is_increasing && !check_inc_valid(level)) || (!is_increasing && !check_dec_valid(level)) {
        is_safe = false;
    }

    is_safe
}

fn is_diff_valid(level: &Vec<i32>) -> bool {
    let mut valid: bool = true;

    for i in 0..level.len() {
        let diff: i32 = (level[i] - level[i + 1]).abs();
        if diff > 3 || diff < 1 {
            valid = false;
            break;
        }
    }
    valid
}

fn check_inc_valid(level: &Vec<i32>) -> bool {
    let mut valid: bool = true;
    let mut curr: i32 = 0;
    while curr != (level.len() as i32) && level[curr as usize] < level[(curr + 1) as usize] {
        curr += 1;
    }

    if curr != (level.len() as i32) {
        valid = false;
    }
    valid
}

fn check_dec_valid(level: &Vec<i32>) -> bool {
    let mut valid: bool = true;
    let mut curr: i32 = 0;
    while curr != (level.len() as i32) && level[curr as usize] > level[(curr + 1) as usize] {
        curr += 1;
    }

    if curr != (level.len() as i32) {
        valid = false;
    }
    valid
}

pub fn find_safe_levels(reports: &Vec<Vec<i32>>) -> i32 {
    let mut num_safe_lvls: i32 = 0;

    //for level in reports.iter() {
    //if is_level_safe(level) {
    //num_safe_lvls += 1;
    //}
    //}
    println!("{:?}", &reports[reports.len() - 1]);
    if is_level_safe(&reports[reports.len() - 1]) {
        num_safe_lvls += 1;
    }
    num_safe_lvls
}
