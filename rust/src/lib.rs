mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn queens_attackthe_king(queens: &[[i32; 2]], king: [i32; 2]) -> Vec<[i32; 2]> {
    const DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let set = queens.iter().map(|v| [v[0], v[1]]).collect();
    DIRS.into_iter()
        .filter_map(|d| dfs(&set, king[0], king[1], d.0, d.1))
        .map(|v| v.into())
        .collect()
}

fn dfs(queens: &HashSet<[i32; 2]>, x: i32, y: i32, dx: i32, dy: i32) -> Option<[i32; 2]> {
    if (0..8).contains(&x) && (0..8).contains(&y) {
        if queens.contains(&[x, y]) {
            return Some([x, y]);
        }
        dfs(queens, x + dx, y + dy, dx, dy)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            queens_attackthe_king(&[[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]], [0, 0]),
            [[0, 1], [1, 0], [3, 3]],
        );
        sort_eq(
            queens_attackthe_king(
                &[[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],
                [3, 3],
            ),
            [[2, 2], [3, 4], [4, 4]],
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
