#[macro_use] extern crate itertools;

use std::env;
use std::fs;
use std::io;

fn parse(path: &String) -> io::Result<Vec<Vec<u8>>> {
    Ok(fs::read(path)?
        .split(|c| char::from(*c) == '\n')
        .map(Vec::from)
        .collect())
}

fn difference(a: &Vec<u8>, b: &Vec<u8>) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|(x, y)| x != y)
        .count()
}

fn score(id: &Vec<u8>) -> (bool, bool) {
    let mut counts: [u8; 256] = [0; 256];
    id.iter().for_each(|&c| counts[c as usize] += 1);
    (counts.contains(&2), counts.contains(&3))
}

fn count_true(bools: &Vec<bool>) -> usize {
    bools.iter().filter(|&p| *p).count()
}

fn part1(ids: &Vec<Vec<u8>>) -> u32 {
    let (twos, threes): (Vec<_>, Vec<_>) = ids.iter()
        .map(score)
        .unzip();

    (count_true(&twos) * count_true(&threes)) as u32
}

fn part2(ids: &Vec<Vec<u8>>) -> String {
    let (s1, s2) = iproduct!(ids, ids)
        .find(|(a, b)| difference(a, b) == 1)
        .expect("No two IDs with difference == 1");

    let bytes: Vec<_> = s1.iter()
        .zip(s2)
        .filter(|(a, b)| a == b)
        .map(|t| *t.0)
        .collect();

    String::from_utf8(bytes)
        .expect("Could not convert to string")
}

fn main() {
    let path = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Missing input file parameter").to_string();

    let input = parse(&path)
        .expect("Failed to parse input");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
