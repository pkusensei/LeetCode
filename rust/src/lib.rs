mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_pair_strength(nums: Vec<i32>) -> i64 {
    let mut res = 0;
    for (i, &a) in nums.iter().enumerate() {
        for &b in &nums[1 + i..] {
            let v = gcd(a.into(), b.into());
            res = res.max(i64::from(a) * i64::from(b) / v.pow(2));
        }
    }
    res
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
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
