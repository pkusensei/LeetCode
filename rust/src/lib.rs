mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total(nums: &[i32], s: &str) -> i64 {
    let s = s.as_bytes();
    let mut left = 0;
    let mut res = 0;
    for ch in s.chunk_by(|a, b| a == b) {
        if ch[0] == b'1' {
            let right = left + ch.len();
            if left > 0 {
                let sum: i64 = nums[left - 1..right].iter().map(|&v| i64::from(v)).sum();
                let min = i64::from(*nums[left - 1..right].iter().min().unwrap_or(&0));
                res += sum - min;
            } else {
                res += nums[..right].iter().map(|&v| i64::from(v)).sum::<i64>();
            }
        }
        left += ch.len();
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
        assert_eq!(max_total(&[9, 3, 5], "011"), 14);
    }

    #[test]
    fn test() {
        assert_eq!(max_total(&[11, 2, 9], "101"), 20);
    }
}
