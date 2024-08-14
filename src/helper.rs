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

type Coord = (usize, usize);

fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
    [
        (a.saturating_sub(1), b),
        (a + 1, b),
        (a, b.saturating_sub(1)),
        (a, b + 1),
    ]
    .into_iter()
    .filter(move |&p| p != (a, b))
}
