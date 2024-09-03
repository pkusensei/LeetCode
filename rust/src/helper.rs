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

pub fn around(x: i32, y: i32) -> impl Iterator<Item = Coord> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter_map(|(x, y)| {
        if x >= 0 && y >= 0 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
}

pub fn get_dimensions<T, U: AsRef<[T]>>(grid: &[U]) -> (usize, usize) {
    let (row, col) = (
        grid.len(),
        grid.first().map(|r| r.as_ref().len()).unwrap_or(0),
    );
    (row, col)
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Rem<Output = T> + PartialEq + From<bool>,
{
    if a == false.into() {
        b
    } else {
        gcd(b % a, a)
    }
}
