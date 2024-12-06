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
    }

    sel_sort(&mut left_list);
    sel_sort(&mut right_list);

    let dist = calc_distance(&mut left_list, &mut right_list);
    println!("{}", dist);
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn sel_sort(list: &mut Vec<i32>) {
    for i in 0..list.len() {
        let mut min_idx = i;
        for j in i + 1..list.len() {
            if list[j] < list[min_idx] {
                min_idx = j;
            }
        }

        if i != min_idx {
            list.swap(i, min_idx);
        }
    }
}

pub fn calc_distance(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
    let len = if left_list.len() == right_list.len() {
        left_list.len()
    } else {
        panic!("left and right list arent the same length!");
    };

    let mut dist: i32 = 0;
    for i in 0..len {
        dist = dist + (left_list[i] - right_list[i]).abs();
    }

    dist
}
