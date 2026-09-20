mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// a - b + c - d
// (a c d b) a - c + d - b
// (a b d c) a - b + d - c
// (a c b d) a - c + b - d
pub fn max_value(nums: &[i32]) -> i64 {
    let mut pulse = 0;
    let mut max_even = 0;
    let mut max_odd = i64::MIN >> 2;
    let mut min = i64::MAX;
    for (i, &num) in nums.iter().enumerate() {
        if i & 1 == 0 {
            pulse += i64::from(num);
            // length is odd
            min = min.min(pulse - max_odd);
            max_odd = max_odd.max(pulse);
        } else {
            pulse -= i64::from(num);
            min = min.min(pulse - max_even);
            max_even = max_even.max(pulse);
        };
    }
    pulse.max(pulse - 2 * min)
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
    fn basics() {
        assert_eq!(max_value(&[9, 7]), 2);
    }

    #[test]
    fn test() {}
}
