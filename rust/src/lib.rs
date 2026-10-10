mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn resilient_subarray(nums: &[i32], k: i32) -> i32 {
    let mut res = 1;
    for ch in nums.chunk_by(|a, b| a % k == b % k) {
        let len = ch.len() as i32;
        let rem = ch[0] % k;
        let sum = len * rem;
        if (sum - rem).rem_euclid(k) == 0 {
            res = res.max(len);
        } else {
            let mut sum = sum;
            for i in (0..len).rev() {
                sum -= rem;
                if sum.rem_euclid(k) == 0 {
                    res = res.max(1 + i);
                    break;
                }
            }
        }
    }
    res
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
    fn test() {
        assert_eq!(resilient_subarray(&[27, 9, 45, 63, 63, 63, 24, 18], 18), 5);
        assert_eq!(resilient_subarray(&[5, 5], 8), 1);
    }
}
