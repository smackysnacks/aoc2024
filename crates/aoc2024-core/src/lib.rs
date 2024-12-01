pub fn iter_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines()
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.into()).collect()
}
