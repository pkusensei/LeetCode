mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    let cols = board[0].len();
    let mut res = i64::MIN;
    let arr = board.iter().map(|r| big3(r.iter().copied())).collect_vec();
    for (row, vals) in arr.iter().enumerate() {
        let (col, val) = vals[0];
        // for &(col, val) in vals {
        // (max, row, col) in each column
        let mut col_maxs = vec![[(i32::MIN, 0, cols); 2]; cols];
        for (r, _v) in arr.iter().enumerate().filter(|&(r, _)| r != row) {
            for &(c, v) in _v.iter().filter(|&&(c, _)| c != col) {
                if v >= col_maxs[c][0].0 {
                    col_maxs[c][1] = col_maxs[c][0];
                    col_maxs[c][0] = (v, r, c);
                } else if v > col_maxs[c][1].0 {
                    col_maxs[c][1] = (v, r, c);
                }
            }
        }
        let Some(temp) = col_maxs
            .iter()
            .flatten()
            .array_combinations::<2>()
            .filter_map(|[a, b]| {
                if a.1 != b.1 && a.2 != b.2 {
                    Some(i64::from(a.0) + i64::from(b.0))
                } else {
                    None
                }
            })
            .max()
        else {
            unreachable!();
        };
        res = res.max(i64::from(val) + temp);
        // }
    }
    res
}

fn big3(it: impl Iterator<Item = i32>) -> [(usize, i32); 3] {
    let [mut i1, mut i2, mut i3] = [0; 3];
    let [mut v1, mut v2, mut v3] = [i32::MIN; 3];
    for (i, num) in it.enumerate() {
        if num >= v1 {
            (i3, v3) = (i2, v2);
            (i2, v2) = (i1, v1);
            (i1, v1) = (i, num);
        } else if num >= v2 {
            (i3, v3) = (i2, v2);
            (i2, v2) = (i, num);
        } else if num > v3 {
            (i3, v3) = (i, num);
        }
    }
    [(i1, v1), (i2, v2), (i3, v3)]
}

pub fn only11(board: Vec<Vec<i32>>) -> i64 {
    let cols = board[0].len();
    let mut row_set = HashSet::new();
    for (r, row) in board.iter().enumerate() {
        let vals = big3(row.iter().copied());
        row_set.extend(vals.into_iter().map(|(col, v)| (v, r, col)));
    }
    let mut col_set = HashSet::new();
    for c in 0..cols {
        let vals = big3(board.iter().map(|r| r[c]));
        col_set.extend(vals.into_iter().map(|(row, v)| (v, row, c)));
    }
    let mut res = i64::MIN;
    for v in row_set
        .intersection(&col_set)
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0))
        .take(11)
        .array_combinations::<3>()
    {
        let [a, b, c] = v;
        if a.2 != b.2 && a.2 != c.2 && b.2 != c.2 && a.1 != b.1 && b.1 != c.1 && c.1 != a.1 {
            res = res.max([a.0, b.0, c.0].map(i64::from).iter().sum());
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(
            maximum_value_sum(vec![
                vec![-38, -83, 3],
                vec![-31, -50, 10], // another -31 !!!
                vec![-99, -49, 32],
                vec![-8, -25, 46],  // -8
                vec![-85, -51, 44], // 44
                vec![-67, -82, 28],
                vec![-18, -31, -65] // -31
            ]),
            5
        );
        assert_eq!(
            maximum_value_sum(vec![
                vec![49685527, 202668489, -124061990, -197137708],
                vec![495839479, 978987221, -588255976, 732922326],
                vec![207406008, -851023502, 725053940, 311164335],
                vec![-422548014, 110352941, 302829364, 174642760]
            ]),
            1878683921
        );

        assert_eq!(
            only11(vec![
                vec![-38, -83, 3],
                vec![-31, -50, 10], // another -31 !!!
                vec![-99, -49, 32],
                vec![-8, -25, 46],  // -8
                vec![-85, -51, 44], // 44
                vec![-67, -82, 28],
                vec![-18, -31, -65] // -31
            ]),
            5
        );
        assert_eq!(
            only11(vec![
                vec![49685527, 202668489, -124061990, -197137708],
                vec![495839479, 978987221, -588255976, 732922326],
                vec![207406008, -851023502, 725053940, 311164335],
                vec![-422548014, 110352941, 302829364, 174642760]
            ]),
            1878683921
        );
    }
}
