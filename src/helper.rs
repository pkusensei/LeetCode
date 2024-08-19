#![allow(dead_code)]

fn is_palindrome(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }
    s.bytes()
        .rev()
        .zip(s.bytes().take(s.len() / 2 + 1))
        .all(|(b1, b2)| b1 == b2)
}

pub type Coord = (usize, usize);

pub fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
    [
        (a.saturating_sub(1), b),
        (a + 1, b),
        (a, b.saturating_sub(1)),
        (a, b + 1),
    ]
    .into_iter()
    .filter(move |&p| p != (a, b))
}

pub fn get_dimensions<T, U: AsRef<[T]>>(grid: &[U]) -> (usize, usize) {
    let (row, col) = (
        grid.len(),
        grid.first().map(|r| r.as_ref().len()).unwrap_or(0),
    );
    (row, col)
}
