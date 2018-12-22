use std::str::FromStr;
use std::fmt::Debug;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Result};

pub fn parse_checked<T>(s: &str) -> T
    where T: FromStr, <T as FromStr>::Err: Debug {
    s.parse::<T>().expect("Failed to parse")
}

pub fn read_arg_file_lines() -> impl Iterator<Item = String> {
    let path = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Missing input file parameter").to_string();

    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
}