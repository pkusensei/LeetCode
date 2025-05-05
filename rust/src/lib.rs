mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for &num in &nums1 {
        for v in 1..=num.isqrt() {
            if num % v == 0 {
                *map.entry(v).or_insert(0) += 1;
                if num / v != v {
                    *map.entry(num / v).or_insert(0) += 1;
                }
            }
        }
    }
    nums2
        .iter()
        .map(|num| i64::from(*map.get(&(num * k)).unwrap_or(&0)))
        .sum()
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
