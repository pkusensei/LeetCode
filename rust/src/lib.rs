mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn length_of_lis(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut res = 1;
    for right in 1..n {
        for left in 0..right {
            if nums[left] < nums[right] {
                dp[right] = dp[right].max(1 + dp[left]);
                res = res.max(dp[right]);
            }
        }
    }
    res
}

pub fn with_bianry_search(nums: &[i32]) -> i32 {
    let mut lis = vec![];
    for &num in nums.iter() {
        let i = lis.partition_point(|&v| v < num);
        if i == lis.len() {
            lis.push(num);
        } else {
            lis[i] = num;
        }
    }
    lis.len() as i32
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
        assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(length_of_lis(&[7, 7, 7, 7, 7, 7, 7]), 1);

        assert_eq!(with_bianry_search(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(with_bianry_search(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(with_bianry_search(&[7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test() {}
}
