#![allow(dead_code)]

pub fn is_palindrome<I>(it: I) -> bool
where
    I: Clone + IntoIterator,
    I::Item: PartialEq,
    I::IntoIter: DoubleEndedIterator,
{
    it.clone()
        .into_iter()
        .zip(it.into_iter().rev())
        .all(|(a, b)| a == b)
}

pub fn get_dimensions<T, U: AsRef<[T]>>(grid: &[U]) -> [usize; 2] {
    let (row, col) = (
        grid.len(),
        grid.first().map(|r| r.as_ref().len()).unwrap_or(0),
    );
    [row, col]
}

pub type Coord = [usize; 2];

pub fn neighbors([a, b]: Coord) -> impl Iterator<Item = Coord> {
    [
        [a.saturating_sub(1), b],
        [a + 1, b],
        [a, b.saturating_sub(1)],
        [a, b + 1],
    ]
    .into_iter()
    .filter(move |&p| p != [a, b])
}

pub const ALL_DIRS: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

pub fn around(x: i32, y: i32) -> impl Iterator<Item = Coord> {
    [
        [x - 1, y - 1],
        [x, y - 1],
        [x + 1, y - 1],
        [x - 1, y],
        [x + 1, y],
        [x - 1, y + 1],
        [x, y + 1],
        [x + 1, y + 1],
    ]
    .into_iter()
    .filter_map(|[x, y]| {
        if x >= 0 && y >= 0 {
            Some([x as usize, y as usize])
        } else {
            None
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::West => Self::North,
            Direction::South => Self::West,
            Direction::East => Self::South,
        }
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Rem<Output = T> + PartialEq + From<bool>,
{
    if a == false.into() { b } else { gcd(b % a, a) }
}

pub const fn mod_pow(mut base: i64, mut exp: i64, modu: i64) -> i64 {
    let mut res = 1;
    base %= modu;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % modu;
        }
        exp /= 2;
        base = base.pow(2) % modu;
    }
    res
}
