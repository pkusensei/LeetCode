mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs[0].len();
    let mut lis = vec![1; n];
    for right in 1..n {
        for left in 0..right {
            if strs.iter().all(|s| {
                let s = s.as_bytes();
                s[left] <= s[right]
            }) {
                lis[right] = lis[right].max(1 + lis[left]);
            }
        }
    }
    n as i32 - lis.iter().max().unwrap_or(&1)
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
