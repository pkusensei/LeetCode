mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good(nums: &[i32], k: i32) -> i64 {
    use std::collections::HashMap;
    let n = nums.len();
    let k = i64::from(k);
    let mut pair_count = 0;
    let mut map = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        let v = map.entry(num).or_insert(0);
        pair_count += *v;
        *v += 1;
        while pair_count >= k {
            res += n - right;
            map.entry(nums[left]).and_modify(|v| *v -= 1);
            pair_count -= map[&nums[left]];
            left += 1;
        }
    }
    res as i64
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
    fn basics() {
        assert_eq!(count_good(&[1, 1, 1, 1, 1], 10), 1);
        assert_eq!(count_good(&[3, 1, 4, 3, 2, 2, 4], 2), 4);
    }

    #[test]
    fn test() {}
}
