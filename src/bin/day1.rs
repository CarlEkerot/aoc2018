use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(path: String) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Failed to read file")
        .split_whitespace()
        .map(|p| p.parse::<i32>().unwrap())
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
    let args: Vec<String> = env::args().collect();
    let path = args.get(1)
        .expect("Missing input file parameter").to_string();

    let input = parse(path);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
