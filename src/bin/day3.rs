extern crate regex;
#[macro_use] extern crate itertools;

use std::env;
use std::fs;
use std::io;
use regex::{Regex, Captures};

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn indices(&self) -> Vec<usize> {
        iproduct!(self.x..(self.x + self.width),
                  self.y..(self.y + self.height))
            .map(|(x, y)| y * 1000 + x)
            .collect()
    }
}

fn parse(path: &String) -> io::Result<Vec<Claim>> {
    fn parse_usize(c: &Captures, i: usize) -> usize {
        c.get(i)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("Failed to parse")
    }

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let text = fs::read_to_string(path)?;

    Ok(re.captures_iter(text.as_str()).map(|c| Claim {
        id: parse_usize(&c, 1),
        x: parse_usize(&c, 2),
        y: parse_usize(&c, 3),
        width: parse_usize(&c, 4),
        height: parse_usize(&c, 5),
    }).collect())
}

fn part1(claims: &Vec<Claim>, fabric: &mut [u32]) -> u32 {
    for c in claims {
        for i in c.indices() {
            fabric[i] += 1;
        }
    }

    fabric.iter()
        .filter(|&c| *c > 1)
        .count() as u32
}

fn part2(claims: &Vec<Claim>, fabric: &mut [u32]) -> usize {
    claims.iter()
        .find(|c| c.indices().iter().all(|&i| fabric[i] == 1))
        .map(|ref c| c.id).unwrap()
}

fn main() {
    let path = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Missing input file parameter").to_string();

    let input = parse(&path)
        .expect("Failed to parse input");

    let mut fabric = [0u32; 1000 * 1000];

    println!("{}", part1(&input, &mut fabric));
    println!("{}", part2(&input, &mut fabric));
}
