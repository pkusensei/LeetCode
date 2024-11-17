mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn oranges_rotting(grid: &mut [&mut [i32]]) -> i32 {
    let mut queue = std::collections::VecDeque::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 2 {
                queue.push_back((x, y));
            }
        }
    }
    let mut res = 0;
    while !queue.is_empty() {
        let n = queue.len();
        res += 1;
        for _ in 0..n {
            let Some((x, y)) = queue.pop_front() else {
                continue;
            };
            for (nx, ny) in neighbors((x, y)) {
                if let Some(v) = grid.get_mut(ny).and_then(|r| r.get_mut(nx)) {
                    if *v == 1 {
                        *v = 2;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    if grid.iter().any(|r| r.iter().any(|&v| v == 1)) {
        -1
    } else {
        (res - 1).max(0)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            oranges_rotting(&mut [&mut [2, 1, 1], &mut [1, 1, 0], &mut [0, 1, 1]]),
            4
        );
        debug_assert_eq!(
            oranges_rotting(&mut [&mut [2, 1, 1], &mut [0, 1, 1], &mut [1, 0, 1]]),
            -1
        );
        debug_assert_eq!(oranges_rotting(&mut [&mut [0, 2]]), 0);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
