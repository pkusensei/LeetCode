mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn create_grid(m: i32, n: i32, k: i32) -> Vec<String> {
    let [rows, cols] = [m, n].map(|v| v as usize);
    if rows == 1 || cols == 1 {
        if k > 1 {
            return vec![];
        }
        return vec![repeat_n('.', cols).collect(); rows];
    }
    let mut res = vec![vec![b'#'; cols]; rows];
    match k {
        1 => {
            for r in 0..rows - 1 {
                res[r][0] = b'.'
            }
            res[rows - 1].fill(b'.');
        }
        2 => {
            for r in 0..rows - 2 {
                res[r][0] = b'.';
            }
            res[rows - 2].fill(b'.');
            res[rows - 1][cols - 2..].fill(b'.');
        }
        3 => {
            if rows >= 2 && cols >= 3 {
                for r in 0..rows - 2 {
                    res[r][0] = b'.';
                }
                res[rows - 2].fill(b'.');
                res[rows - 1][cols - 3..].fill(b'.');
            } else if rows >= 3 && cols >= 2 {
                for r in 0..rows - 3 {
                    res[r][0] = b'.';
                }
                res[rows - 3].fill(b'.');
                res[rows - 2][cols - 2..].fill(b'.');
                res[rows - 1][cols - 2..].fill(b'.');
            } else {
                res = vec![]
            }
        }
        4 => {
            if rows >= 2 && cols >= 4 {
                for r in 0..rows - 2 {
                    res[r][0] = b'.';
                }
                res[rows - 2].fill(b'.');
                res[rows - 1][cols - 4..].fill(b'.');
            } else if rows >= 4 && cols >= 2 {
                for r in 0..rows - 4 {
                    res[r][0] = b'.';
                }
                res[rows - 4].fill(b'.');
                for r in rows - 3..rows {
                    res[r][cols - 2..].fill(b'.');
                }
            } else if rows >= 3 && cols >= 3 {
                for r in 0..rows - 3 {
                    res[r][0] = b'.';
                }
                res[rows - 3][..cols - 1].fill(b'.');
                res[rows - 2][cols - 3..].fill(b'.');
                res[rows - 1][cols - 2..].fill(b'.');
            } else {
                res = vec![]
            }
        }
        _ => unreachable!(),
    };
    res.into_iter()
        .map(|v| String::from_utf8(v).unwrap())
        .collect()
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn test() {}
}
