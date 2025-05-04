mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(grid: &[&[i32]]) -> i32 {
    let n = grid[0].len();
    let mut curr = vec![i32::MAX; n];
    let mut res = i32::MIN;
    for row in grid.iter() {
        for (c, &v) in row.iter().enumerate() {
            if c > 0 {
                curr[c] = curr[c].min(curr[c - 1]);
            }
            res = res.max(v - curr[c]);
            curr[c] = curr[c].min(v);
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
    fn basics() {
        assert_eq!(
            max_score(&[&[9, 5, 7, 3], &[8, 9, 6, 1], &[6, 7, 14, 3], &[2, 5, 3, 1]]),
            9
        );
        assert_eq!(max_score(&[&[4, 3, 2], &[3, 2, 1]]), -1);
    }

    #[test]
    fn test() {}
}
