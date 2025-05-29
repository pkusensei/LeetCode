mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let k = k as usize;
    let mut vals = vec![i64::MAX / 2; k];
    vals[k - 1] = 0; // Before any num added, base case prefix sum == 0
    let mut res = i64::MIN / 2;
    let mut prefix = 0;
    for (i, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        prefix += num;
        res = res.max(prefix - vals[i % k]);
        vals[i % k] = vals[i % k].min(prefix);
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
        assert_eq!(max_subarray_sum(&[1, 2], 1), 3);
        assert_eq!(max_subarray_sum(&[-1, -2, -3, -4, -5], 4), -10);
        assert_eq!(max_subarray_sum(&[-5, 1, 2, -3, 4], 2), 4);
    }

    #[test]
    fn test() {}
}
