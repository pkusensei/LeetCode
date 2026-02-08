mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn dominant_indices(nums: &[i32]) -> i32 {
    let mut res = 0;
    let mut sum = f64::from(*nums.last().unwrap());
    let mut count = 1.0;
    for &num in nums.iter().rev().skip(1) {
        res += i32::from(f64::from(num) > sum / count);
        sum += f64::from(num);
        count += 1.0;
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
        assert_eq!(dominant_indices(&[5, 4, 3]), 2);
    }

    #[test]
    fn test() {}
}
