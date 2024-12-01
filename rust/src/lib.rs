mod dsu;
mod helper;
mod trie;

use std::{collections::HashMap, iter};

#[allow(unused_imports)]
use helper::*;

pub fn alphabet_board_path(target: &str) -> String {
    const BOARD: [&str; 6] = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"];
    let mut board = HashMap::new();
    for (y, row) in BOARD.iter().enumerate() {
        for (x, b) in row.bytes().enumerate() {
            board.insert(b, (x, y));
        }
    }
    let mut res = vec![];
    let (mut x, mut y) = (0, 0);
    for &(nx, ny) in target.bytes().filter_map(|b| board.get(&b)) {
        use std::cmp::Ordering::*;
        // L D U R to stay on board
        match (x.cmp(&nx), y.cmp(&ny)) {
            (Less, Less) => {
                res.extend(iter::repeat(b'D').take(y.abs_diff(ny)));
                res.extend(iter::repeat(b'R').take(x.abs_diff(nx)));
            }
            (Less, Equal) => res.extend(iter::repeat(b'R').take(x.abs_diff(nx))),
            (Less, Greater) => {
                res.extend(iter::repeat(b'U').take(y.abs_diff(ny)));
                res.extend(iter::repeat(b'R').take(x.abs_diff(nx)));
            }
            (Equal, Less) => res.extend(iter::repeat(b'D').take(y.abs_diff(ny))),
            (Equal, Equal) => (),
            (Equal, Greater) => res.extend(iter::repeat(b'U').take(y.abs_diff(ny))),
            (Greater, Less) => {
                res.extend(iter::repeat(b'L').take(x.abs_diff(nx)));
                res.extend(iter::repeat(b'D').take(y.abs_diff(ny)));
            }
            (Greater, Equal) => res.extend(iter::repeat(b'L').take(x.abs_diff(nx))),
            (Greater, Greater) => {
                res.extend(iter::repeat(b'L').take(x.abs_diff(nx)));
                res.extend(iter::repeat(b'U').take(y.abs_diff(ny)));
            }
        }
        res.push(b'!');
        x = nx;
        y = ny;
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(alphabet_board_path("leet"), "DDR!UURRR!!DDD!");
        debug_assert_eq!(alphabet_board_path("code"), "RR!DDRR!LUU!R!");
    }

    #[test]
    fn test() {
        debug_assert_eq!(alphabet_board_path("zdz"), "DDDDD!UUUUURRR!LLLDDDDD!");
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
