use std::fs;

pub fn input(filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();
    return contents;
}

pub fn input_vec(contents: &String) -> Vec<&str> {
    let result: Vec<_> = contents.lines().collect();
    return result;
}