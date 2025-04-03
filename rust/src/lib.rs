mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_strength(mut nums: Vec<i32>) -> i64 {
    let n = nums.len();
    nums.sort_unstable();
    let mut res = 0;
    let mut idx = 0;
    while 1 + idx < n && nums[idx] < 0 && nums[1 + idx] < 0 {
        if res == 0 {
            res = 1;
        }
        res *= i64::from(nums[idx] * nums[1 + idx]);
        idx += 2;
    }
    while idx < n && nums[idx] <= 0 {
        idx += 1;
    }
    while let Some(&v) = nums.get(idx) {
        if res == 0 {
            res = 1;
        }
        res *= i64::from(v);
        idx += 1;
    }
    if res == 0 { nums[n - 1] as i64 } else { res }
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
