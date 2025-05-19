mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    dfs(a, b, 0, 0, &mut vec![vec![None; n]; 4])
}

fn dfs(a: &[i32], b: &[i32], i1: usize, i2: usize, memo: &mut [Vec<Option<i64>>]) -> i64 {
    if i1 >= a.len() {
        return 0;
    }
    if i2 >= b.len() {
        return i64::MIN / 2;
    }
    if let Some(v) = memo[i1][i2] {
        return v;
    }
    let take = i64::from(a[i1]) * i64::from(b[i2]) + dfs(a, b, 1 + i1, 1 + i2, memo);
    let skip = dfs(a, b, i1, 1 + i2, memo);
    let res = take.max(skip);
    memo[i1][i2] = Some(res);
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
        assert_eq!(max_score(&[3, 2, 5, 6], &[2, -6, 4, -5, -3, 2, -7]), 26);
        assert_eq!(max_score(&[-1, 4, 5, -2], &[-5, -1, -3, -2, -4]), -1);
    }

    #[test]
    fn test() {}
}
