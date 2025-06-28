mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_gcd_score(nums: &[i32], k: i32) -> i64 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }
    let mut res = 0;
    for (i1, &a) in nums.iter().enumerate() {
        let mut gcd_ = a;
        let mut count = 0;
        let mut lowest = i32::MAX;
        for (i2, &b) in nums.iter().enumerate().skip(i1) {
            gcd_ = gcd(gcd_, b);
            let low_bit = b & -b;
            if lowest > low_bit {
                lowest = low_bit;
                count = 0;
            }
            if lowest == low_bit {
                count += 1
            }
            let temp = i64::from(gcd_) * if count <= k { 2 } else { 1 };
            res = res.max(temp * (i2 + 1 - i1) as i64);
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
        assert_eq!(max_gcd_score(&[2, 4], 1), 8);
        assert_eq!(max_gcd_score(&[3, 5, 7], 2), 14);
        assert_eq!(max_gcd_score(&[5, 5, 5], 1), 15);
    }

    #[test]
    fn test() {}
}
