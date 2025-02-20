mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut seen = std::collections::HashMap::from([(nums[n - 1], 1)]);
    let mut res = 0;
    for (i1, &c) in nums[..n - 1].iter().enumerate().rev() {
        for (i2, b) in nums[..i1].iter().enumerate().rev() {
            for a in nums[..i2].iter().rev() {
                let d = a + b + c;
                res += seen.get(&d).unwrap_or(&0);
            }
        }
        *seen.entry(c).or_insert(0) += 1;
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
