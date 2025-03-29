mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let mut prefix = 0;
    let mut seen = HashMap::from([(0, 1)]);
    let mut res = 0;
    for &num in nums.iter() {
        prefix ^= num;
        if let Some(v) = seen.get(&(0 ^ prefix)) {
            res += v
        }
        *seen.entry(prefix).or_insert(0) += 1;
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
