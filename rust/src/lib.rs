mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let n = i64::from(n);
    let mut left = 0;
    let mut right: i64 = batteries.iter().map(|&v| i64::from(v)).sum();
    while left < right {
        let mid = left + (right - left + 1) / 2;
        let sum: i64 = batteries.iter().map(|&v| mid.min(i64::from(v))).sum();
        if sum >= mid * n {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
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
