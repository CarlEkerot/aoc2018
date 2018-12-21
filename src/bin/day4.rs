extern crate regex;
extern crate itertools;
extern crate aoc2018;

use std::collections::HashMap;
use std::env;
use std::fs::{File};
use std::io::{BufReader, BufRead, Result};
use regex::Regex;
use aoc2018::parse_checked;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Start(u32),
    Sleep,
    Wake,
}

impl EventType {
    fn parse(text: &str) -> EventType {
        match text {
            "falls asleep" => EventType::Sleep,
            "wakes up" => EventType::Wake,
            s => {
                let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                let cap = re.captures(s).unwrap();
                EventType::Start(parse_checked::<u32>(&cap[1]))
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    event_type: EventType,
}

fn parse(path: &String) -> Result<Vec<Event>> {
    let re = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] (.+)").unwrap();

    let f = File::open(path)?;
    let lines = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap());

    Ok(lines.map(|l| {
        let cap = re.captures(l.as_str()).unwrap();
        Event {
            year: parse_checked::<u16>(&cap[1]),
            month: parse_checked::<u8>(&cap[2]),
            day: parse_checked::<u8>(&cap[3]),
            hour: parse_checked::<u8>(&cap[4]),
            minute: parse_checked::<u8>(&cap[5]),
            event_type: EventType::parse(&cap[6]),
        }
    }).sorted().collect())
}

fn part1(events: &Vec<Event>) -> (u32, u32) {
    let mut guard_minutes: HashMap<u32, [u8; 60]> = HashMap::new();
    let mut guard = 0;
    let mut sleep: Option<&Event> = None;

    for e in events {
        sleep = match e.event_type {
            EventType::Start(id) => {
                guard = id;
                None
            }
            EventType::Wake => {
                sleep.map(|o| {
                    let minutes = guard_minutes.entry(guard)
                        .or_insert([0u8; 60]);
                    for min in o.minute..e.minute {
                        minutes[min as usize] += 1;
                    }
                });
                None
            }
            EventType::Sleep => Some(e)
        }
    }

    fn max_idx(a: &[u8; 60]) -> usize {
        a.iter()
            .enumerate()
            .max_by_key(|(_, &m)| m)
            .unwrap()
            .0
    }

    let part1_id = guard_minutes.iter()
        .max_by_key(|(_, mins)| {
            mins.iter()
                .map(|&m| m as usize)
                .sum::<usize>()
        })
        .unwrap()
        .0;

    let part1_minute = guard_minutes
        .get(part1_id)
        .map(max_idx)
        .unwrap();

    let (part2_id, _) = guard_minutes.iter()
        .max_by_key(|(_, mins)| {
          mins.iter().max().unwrap()
        })
        .unwrap();

    let part2_minute = guard_minutes
        .get(part2_id)
        .map(max_idx)
        .unwrap();

    (*part1_id * part1_minute as u32,
     *part2_id * part2_minute as u32)
}

fn main() {
    let path = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Missing input file parameter").to_string();

    let input = parse(&path)
        .expect("Failed to parse input");

    println!("{:?}", part1(&input));
}
