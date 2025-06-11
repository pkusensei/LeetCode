mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
    let n = original.len();
    let mut low = bounds[0][0];
    let mut high = bounds[0][1];
    for idx in 1..n {
        let d = original[idx] - original[idx - 1];
        low = (low + d).max(bounds[idx][0]);
        high = (high + d).min(bounds[idx][1]);
        if low > high {
            return 0;
        }
    }
    high - low + 1
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
