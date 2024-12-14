mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
    if n == m {
        return 1;
    }
    let [rows, cols] = [n, m].map(|v| v as usize);
    let mut seen = vec![vec![false; cols]; rows];
    let mut res = n * m;
    dfs(&mut seen, 0, 0, 0, &mut res);
    res
}

fn dfs(seen: &mut [Vec<bool>], row: usize, col: usize, count: i32, res: &mut i32) {
    if count >= *res {
        return;
    }
    let (rows, cols) = get_dimensions(seen);
    if row >= rows {
        *res = (*res).min(count);
        return;
    }
    if col >= cols {
        dfs(seen, 1 + row, 0, count, res);
        return;
    }
    if seen[row][col] {
        dfs(seen, row, 1 + col, count, res);
        return;
    }
    let delta = (rows - row).min(cols - col);
    for d in (1..=delta).rev() {
        if is_open(seen, row, col, d) {
            flip(seen, row, col, d);
            dfs(seen, row, col + d, count + 1, res);
            flip(seen, row, col, d);
        }
    }
}

fn is_open(seen: &[Vec<bool>], row: usize, col: usize, d: usize) -> bool {
    seen[row..row + d]
        .iter()
        .all(|r| r[col..col + d].iter().all(|&v| !v))
}

fn flip(seen: &mut [Vec<bool>], row: usize, col: usize, d: usize) {
    for r in seen[row..row + d].iter_mut() {
        for v in r[col..col + d].iter_mut() {
            (*v) = !(*v);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(tiling_rectangle(2, 3), 3);
        assert_eq!(tiling_rectangle(5, 8), 5);
        assert_eq!(tiling_rectangle(11, 13), 6);
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
