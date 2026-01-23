mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut res = 0;
    let mut idx = 0;
    while idx < n {
        let start = idx;
        while arr.get(1 + idx).is_some_and(|&v| v > arr[idx]) {
            idx += 1;
        }
        if start == idx {
            idx += 1;
            continue;
        }
        let mid = idx;
        while arr.get(1 + idx).is_some_and(|&v| v < arr[idx]) {
            idx += 1;
        }
        if mid == idx {
            idx += 1;
            continue;
        }
        res = res.max(idx + 1 - start);
    }
    res as i32
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
