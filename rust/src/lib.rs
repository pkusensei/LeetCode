mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 1 {
        return if k & 1 == 1 { -1 } else { nums[0] };
    }
    if k <= 1 {
        return nums[k as usize];
    }
    let mut res = 0;
    for i in 0..(k as usize - 1).min(nums.len()) {
        res = res.max(nums[i])
    }
    if let Some(&v) = nums.get(k as usize) {
        res = res.max(v)
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
