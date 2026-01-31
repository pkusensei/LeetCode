mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_k(nums: &[i32]) -> i32 {
    let mut left = 1;
    let mut right = i64::from(i32::MAX >> 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if non_positive(&nums, mid) <= mid.pow(2) {
            right = mid;
        } else {
            left = 1 + mid
        }
    }
    left as i32
}

fn non_positive(nums: &[i32], k: i64) -> i64 {
    nums.iter()
        .map(|&num| {
            let num = i64::from(num);
            num / k + i64::from(num % k > 0)
        })
        .sum()
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(minimum_k(&[1, 1]), 2);
    }
}
