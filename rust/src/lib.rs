mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rob(nums: &[i32]) -> i32 {
    if nums.len() < 2 {
        return *nums.get(0).unwrap_or(&0);
    }
    let mut dp0 = nums[0];
    let mut dp1 = nums[1];
    for &num in &nums[2..] {
        let curr = dp1.max(num + dp0);
        dp0 = dp0.max(dp1);
        dp1 = curr;
    }
    dp0.max(dp1)
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
    fn basics() {
        assert_eq!(rob(&[2, 1, 1, 2]), 4);
    }

    #[test]
    fn test() {}
}
