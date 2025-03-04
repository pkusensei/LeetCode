mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(nums: &[i32], k: i32) -> i64 {
    const fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let k = i64::from(k);
    let mut count = std::collections::HashMap::new();
    let mut res = 0;
    for &num in nums.iter() {
        let curr = gcd(i64::from(num), k);
        for (i, v) in count.iter() {
            if i * curr % k == 0 {
                res += v;
            }
        }
        *count.entry(curr).or_insert(0) += 1;
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
    fn basics() {
        assert_eq!(count_pairs(&[1, 2, 3, 4, 5], 2), 7);
    }

    #[test]
    fn test() {}
}
