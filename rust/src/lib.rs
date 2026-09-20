mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_intersecting_intervals(mut intervals: Vec<[i32; 2]>) -> i64 {
    intervals.sort_unstable();
    let mut res = 0;
    for (idx, v) in intervals.iter().enumerate() {
        let end = v[1];
        let i = intervals.partition_point(|v| v[0] <= end);
        res += (i - idx - 1) as i64
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {
        assert_eq!(
            count_intersecting_intervals(vec![[1, 2], [2, 3], [3, 4]]),
            2
        );
    }

    #[test]
    fn test() {}
}
