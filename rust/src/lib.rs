mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations_to_make_median_k(mut nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    nums.sort_unstable();
    let med = nums[n / 2];
    match med.cmp(&k) {
        std::cmp::Ordering::Less => {
            let mut res = 0;
            for &num in &nums[n / 2..] {
                if num < k {
                    res += i64::from(k - num);
                } else {
                    break;
                }
            }
            res
        }
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => {
            let mut res = 0;
            for &num in nums[..=n / 2].iter().rev() {
                if num > k {
                    res += i64::from(num - k);
                } else {
                    break;
                }
            }
            res
        }
    }
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
