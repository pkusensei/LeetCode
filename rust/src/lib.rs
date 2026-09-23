mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    if sum == x {
        return n as i32;
    }
    let mut left = 0;
    let mut res = None;
    let mut curr = 0;
    for (right, &num) in nums.iter().enumerate() {
        curr += num;
        while left <= right && curr > sum - x {
            curr -= nums[left];
            left += 1;
        }
        if curr == sum - x {
            res = res.max(Some(1 + right - left))
        }
    }
    if let Some(v) = res {
        (n - v) as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
