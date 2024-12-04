pub mod counter;

use std::str::FromStr;

pub use counter::*;

pub fn lines_iter<T: AsRef<str> + ?Sized>(s: &T) -> impl Iterator<Item = &str> {
    s.as_ref().lines()
}

pub fn lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

pub fn lines_chars(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

pub fn parse<F: FromStr<Err: std::fmt::Debug>>(s: &str) -> F {
    s.parse().unwrap()
}
