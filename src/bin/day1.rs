extern crate aoc2018;

use std::collections::HashSet;
use aoc2018::{read_arg_file_lines, parse_checked};

fn parse() -> Vec<i32> {
    read_arg_file_lines()
        .map(|p| parse_checked::<i32>(p.as_str()))
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |z, x| z + x)
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut seen = HashSet::new();

    let res = input.iter()
        .cycle()
        .scan(0, |z, x| {
            *z += x;
            Some((*z, seen.insert(*z)))
        })
        .find(|(_, is_new)| !is_new)
        .unwrap();

    res.0
}

fn main() {
    let input = parse();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
