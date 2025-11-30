mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    use std::collections::HashMap;
    let sum = nums.iter().fold(0, |acc, v| (acc + v) % p);
    if sum == 0 {
        return 0;
    }
    let n = nums.len();
    let mut map = HashMap::from([(0, -1)]);
    let mut prefix = 0;
    let mut res = n as i32;
    for (idx, num) in nums.iter().enumerate() {
        prefix = (prefix + num) % p;
        let v = (prefix - sum).rem_euclid(p);
        if let Some(prev) = map.get(&v) {
            res = res.min(idx as i32 - prev);
        }
        map.insert(prefix, idx as i32);
    }
    if res == n as i32 { -1 } else { res }
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
