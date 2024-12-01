pub mod counter;

pub use counter::*;

pub fn iter_lines<T: AsRef<str> + ?Sized>(s: &T) -> impl Iterator<Item = &str> {
    s.as_ref().lines()
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.into()).collect()
}
