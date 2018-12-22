extern crate regex;
#[macro_use] extern crate itertools;
extern crate aoc2018;

use std::io;
use regex::Regex;
use aoc2018::parse_checked;
use aoc2018::read_arg_file_lines;

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

fn parse() -> io::Result<Vec<Claim>> {
    let lines = read_arg_file_lines();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    Ok(lines.map(|l| {
        let cap = re.captures(l.as_str())
            .expect("Failed to parse line");
        Claim {
            id: parse_checked::<usize>(&cap[1]),
            x: parse_checked::<usize>(&cap[2]),
            y: parse_checked::<usize>(&cap[3]),
            width: parse_checked::<usize>(&cap[4]),
            height: parse_checked::<usize>(&cap[5]),
        }
    })
    .collect())
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
    let input = parse()
        .expect("Failed to parse input");

    let mut fabric = [0u32; 1000 * 1000];

    println!("{}", part1(&input, &mut fabric));
    println!("{}", part2(&input, &mut fabric));
}
