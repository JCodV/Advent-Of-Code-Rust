use std::fs::read_to_string;

pub fn run_day1(path: &str) {
    let lines: Vec<String> = read_lines(path);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines.iter() {
        let left_num: i32 = line[0..5].parse().expect("Could not parse left_num");
        let right_num: i32 = line[8..13].parse().expect("Could not parse right_num");

        left_list.push(left_num);
        right_list.push(right_num);
        //println!("{}", line);
    }

    for left in left_list.iter() {
        //println!("{}", left);
    }

    sort(&mut left_list);
}

pub fn sort(list: &mut Vec<i32>) {
    // selection sort vec not too large
    let curr: i32 = 0;
    let min: i32 = 0;

    for (i, value) in list.iter().enumerate() {
        for (j, value_j) in list[i + 1..].iter().enumerate() {
            println!("i:{}, j:{}", i, j);
        }
    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
