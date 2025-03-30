mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let s1: HashSet<_> = nums1.iter().copied().collect();
    let s2: HashSet<_> = nums2.iter().copied().collect();
    if let Some(&v) = s1.intersection(&s2).min() {
        return v;
    }
    let [a, b] = [s1, s2].map(|s| s.into_iter().min().unwrap_or(9));
    a.min(b) * 10 + a.max(b)
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
