mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn count_servers(grid: &[&[i32]]) -> i32 {
    let mut xs = HashMap::<_, Vec<_>>::new();
    let mut ys = HashMap::<_, Vec<_>>::new();
    for (row, r) in grid.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            if v == 1 {
                let [x, y] = [col, row].map(|v| v as u8);
                xs.entry(x).or_default().push(y);
                ys.entry(y).or_default().push(x);
            }
        }
    }
    let mut seen = HashSet::new();
    let mut res = 0;
    for (row, r) in grid.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            if v == 1 {
                let [x, y] = [col, row].map(|v| v as u8);
                res += bfs([x, y], &xs, &ys, &mut seen);
            }
        }
    }
    res
}

fn bfs(
    curr: [u8; 2],
    xs: &HashMap<u8, Vec<u8>>,
    ys: &HashMap<u8, Vec<u8>>,
    seen: &mut HashSet<[u8; 2]>,
) -> i32 {
    if !seen.insert(curr) {
        return 0;
    }
    let mut queue = VecDeque::from([curr]);
    let mut res = 0;
    while let Some([curr_x, curr_y]) = queue.pop_front() {
        res += 1;
        for &y in xs[&curr_x].iter() {
            if seen.insert([curr_x, y]) {
                queue.push_back([curr_x, y]);
            }
        }
        for &x in ys[&curr_y].iter() {
            if seen.insert([x, curr_y]) {
                queue.push_back([x, curr_y]);
            }
        }
    }
    if res > 1 {
        res
    } else {
        0
    }
}

fn without_bfs(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let mut row_count = vec![0; rows];
    let mut col_count = vec![0; cols];
    for (row, r) in grid.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            row_count[row] += v;
            col_count[col] += v;
        }
    }
    let mut res = 0;
    for (row, r) in grid.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            if v == 1 && (row_count[row] > 1 || col_count[col] > 1) {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(without_bfs(&[&[1, 0], &[0, 1]]), 0);
        assert_eq!(without_bfs(&[&[1, 0], &[1, 1]]), 3);
        assert_eq!(
            without_bfs(&[&[1, 1, 0, 0], &[0, 0, 1, 0], &[0, 0, 1, 0], &[0, 0, 0, 1]]),
            4
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
