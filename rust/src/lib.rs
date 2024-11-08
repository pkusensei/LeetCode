mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn shortest_bridge(grid: &[&[i32]]) -> i32 {
    let [mut island, mut front] = [0, 1].map(|_| HashSet::new());
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 1 {
                let mut queue = VecDeque::from([(x, y)]);
                island.insert((x, y));
                while let Some((cx, cy)) = queue.pop_front() {
                    for (nx, ny) in neighbors((cx, cy)) {
                        if let Some(&nv) = grid.get(ny).and_then(|r| r.get(nx)) {
                            if nv == 0 {
                                front.insert((nx, ny));
                            } else if island.insert((nx, ny)) {
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
                break 'outer;
            }
        }
    }

    let mut queue: VecDeque<_> = front.iter().map(|&(x, y)| (x, y, 0)).collect();
    while !queue.is_empty() {
        let n = queue.len();
        for _ in 0..n {
            let Some((x, y, dist)) = queue.pop_front() else {
                continue;
            };
            for (nx, ny) in neighbors((x, y)) {
                if let Some(&nv) = grid.get(ny).and_then(|r| r.get(nx)) {
                    if nv == 0 && front.insert((nx, ny)) {
                        queue.push_back((nx, ny, 1 + dist));
                    } else if nv == 1 && !island.contains(&(nx, ny)) {
                        return 1 + dist;
                    }
                }
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
        debug_assert_eq!(shortest_bridge(&[&[0, 1], &[1, 0]]), 1);
        debug_assert_eq!(shortest_bridge(&[&[0, 1, 0], &[0, 0, 0], &[0, 0, 1]]), 2);
        debug_assert_eq!(
            shortest_bridge(&[
                &[1, 1, 1, 1, 1],
                &[1, 0, 0, 0, 1],
                &[1, 0, 1, 0, 1],
                &[1, 0, 0, 0, 1],
                &[1, 1, 1, 1, 1]
            ]),
            1
        );
    }

    #[test]
    fn test() {}

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
