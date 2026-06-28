mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let k = i64::from(k);
    let mut res = i64::MIN;
    let [mut prev_noop, mut prev_mul, mut prev_div, mut prev_past] = [0; 4];
    for num in nums.iter().map(|&v| i64::from(v)) {
        let curr_noop = num.max(num + prev_noop);
        let curr_mul = (num * k).max(num * k + prev_noop.max(prev_mul));
        let curr_div = (num / k).max(num / k + prev_noop.max(prev_div));
        let curr_past = num.max(num + prev_mul.max(prev_div).max(prev_past));
        res = res.max(curr_mul).max(curr_div).max(curr_past);
        [prev_noop, prev_mul, prev_div, prev_past] = [curr_noop, curr_mul, curr_div, curr_past];
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
        assert_eq!(max_subarray_sum(&[1, -2, 3, 4, -5], 2), 14);
    }

    #[test]
    fn test() {}
}
