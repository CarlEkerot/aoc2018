use std::str::FromStr;
use std::fmt::Debug;

pub fn parse_checked<T>(s: &str) -> T
    where T: FromStr, <T as FromStr>::Err: Debug {
    s.parse::<T>().expect("Failed to parse")
}