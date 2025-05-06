mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut curr = vec![1; n];
    for _ in 0..k {
        let prefix = curr.iter().fold(Vec::with_capacity(n), |mut acc, &v| {
            acc.push((v + acc.last().unwrap_or(&0)) % 1_000_000_007);
            acc
        });
        curr = prefix;
    }
    curr[n - 1]
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
    fn test() {}
}
