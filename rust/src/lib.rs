mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let mut seen = vec![vec![-1; n]; n];
    let mut dist = 0;
    let mut queue = std::collections::VecDeque::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 1 {
                seen[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }
    while !queue.is_empty() {
        dist += 1;
        let _len = queue.len();
        for _ in 0.._len {
            let Some((x, y)) = queue.pop_front() else {
                continue;
            };
            for (nx, ny) in neighbors((x, y)) {
                if grid
                    .get(ny)
                    .is_some_and(|row| row.get(nx).is_some_and(|&v| v == 0))
                    && seen[ny][nx] == -1
                {
                    seen[ny][nx] = dist;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    seen.into_iter()
        .flatten()
        .max()
        .filter(|&v| v > 0)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_distance(&[&[1, 0, 1], &[0, 0, 0], &[1, 0, 1]]), 2);
        assert_eq!(max_distance(&[&[1, 0, 0], &[0, 0, 0], &[0, 0, 0]]), 4);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
