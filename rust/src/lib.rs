mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
    use std::collections::HashMap;
    let [m, k] = [m, k].map(|v| v as usize);
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        sum += i64::from(num);
        *map.entry(num).or_insert(0) += 1;
        if idx >= k {
            let left = nums[idx - k];
            let v = map.entry(left).or_insert(0);
            *v -= 1;
            if *v == 0 {
                map.remove(&left);
            }
            sum -= i64::from(left);
        }
        if idx >= k - 1 && map.len() >= m {
            res = res.max(sum);
        }
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
