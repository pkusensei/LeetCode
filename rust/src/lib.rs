mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path_all_keys(grid: &[&str]) -> i32 {
    let mut start = (0, 0);
    let mut k = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, b) in row.bytes().enumerate() {
            if b == b'@' {
                start = (x, y);
            }
            if b.is_ascii_lowercase() {
                k += 1;
            }
        }
    }
    let mut queue = VecDeque::from([(start, 0i32, 0)]);
    let mut seen = HashSet::from([(start, 0)]);
    // ((x, y), bit mask of keys, dist)
    while let Some(((x, y), keys, dist)) = queue.pop_front() {
        if keys.count_ones() == k {
            return dist;
        }
        for (nx, ny) in neighbors((x, y)) {
            let Some(&b) = grid.get(ny).and_then(|row| row.as_bytes().get(nx)) else {
                continue;
            };
            match b {
                b'.' | b'@' => {
                    if seen.insert(((nx, ny), keys)) {
                        queue.push_back(((nx, ny), keys, 1 + dist));
                    }
                }
                b if b.is_ascii_lowercase() => {
                    let nkeys = keys | (1 << (b - b'a'));
                    if seen.insert(((nx, ny), nkeys)) {
                        queue.push_back(((nx, ny), nkeys, 1 + dist));
                    }
                }
                b if b.is_ascii_uppercase() => {
                    if keys & (1 << (b - b'A')) > 0 && seen.insert(((nx, ny), keys)) {
                        queue.push_back(((nx, ny), keys, 1 + dist));
                    }
                }
                _ => (),
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shortest_path_all_keys(&["@.a..", "###.#", "b.A.B"]), 8);
        debug_assert_eq!(shortest_path_all_keys(&["@..aA", "..B#.", "....b"]), 6);
        debug_assert_eq!(shortest_path_all_keys(&["@Aa"]), -1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(shortest_path_all_keys(&["@...a", ".###A", "b.BCc"]), 10);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
