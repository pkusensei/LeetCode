mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path_binary_matrix(grid: &[&[i32]]) -> i32 {
    if grid[0][0] == 1 {
        return -1;
    }
    let n = grid.len();
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    let mut queue = std::collections::VecDeque::from([(0, 0, 1)]);
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == n - 1 && y == n - 1 {
            return dist;
        }
        for (nx, ny) in around(x as i32, y as i32) {
            if grid
                .get(ny)
                .is_some_and(|r| r.get(nx).is_some_and(|&v| v == 0))
                && !seen[ny][nx]
            {
                queue.push_back((nx, ny, 1 + dist));
                seen[ny][nx] = true;
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
        debug_assert_eq!(shortest_path_binary_matrix(&[&[0, 1], &[1, 0]]), 2);
        debug_assert_eq!(
            shortest_path_binary_matrix(&[&[0, 0, 0], &[1, 1, 0], &[1, 1, 0]]),
            4
        );
        debug_assert_eq!(
            shortest_path_binary_matrix(&[&[1, 0, 0], &[1, 1, 0], &[1, 1, 0]]),
            -1
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
