mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut res = n;
    let mut seen = HashMap::new();
    for (idx, &num) in nums.iter().enumerate() {
        if let Some(prev) = seen.get(&num) {
            res = res.min(idx - prev);
        }
        seen.insert(reverse(num), idx);
    }
    if res == n { -1 } else { res as i32 }
}

const fn reverse(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res = res * 10 + num % 10;
        num /= 10;
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
