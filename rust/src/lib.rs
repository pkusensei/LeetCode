mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
    // y-x<x y<2x
    nums.sort_unstable();
    let mut res = 0;
    for (left, &x) in nums.iter().enumerate() {
        let right = nums.partition_point(|&v| v <= 2 * x);
        for &v in nums[1 + left..right].iter() {
            res = res.max(x ^ v)
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
