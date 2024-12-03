pub mod counter;

use std::str::FromStr;

pub use counter::*;

pub fn lines_iter<T: AsRef<str> + ?Sized>(s: &T) -> impl Iterator<Item = &str> {
    s.as_ref().lines()
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.into()).collect()
}

pub fn parse<F: FromStr<Err: std::fmt::Debug>>(s: &str) -> F {
    s.parse().unwrap()
}
