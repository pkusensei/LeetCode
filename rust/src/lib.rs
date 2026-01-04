mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    nums.iter()
        .map(|&num| {
            let mut count = 0;
            let mut res = 0;
            for p in (1..=num.isqrt()).rev() {
                if num % p == 0 {
                    if p * p == num {
                        return 0;
                    } else {
                        count += 2;
                        res += p + num / p;
                    }
                }
            }
            if count == 4 { res } else { 0 }
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
    fn test() {}
}
