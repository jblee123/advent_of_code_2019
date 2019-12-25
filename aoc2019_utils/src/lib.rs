use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

pub fn get_input(filename: &str) -> String {
    let err_msg = format!("Something went wrong reading the input file: {}",
        filename);
    fs::read_to_string(filename).expect(&err_msg)
}
