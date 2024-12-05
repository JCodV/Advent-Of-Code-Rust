use std::fs::read_to_string;

fn main() {
    let test = read_lines("input.txt");
    //println!("{:?}", test);
    let left: Vec<i32> = Vec::new();

    for element in test.iter().enumerate() {
        println!("{:?}", element);
        left.push(element.1);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
