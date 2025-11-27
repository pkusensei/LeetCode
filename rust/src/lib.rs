mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut prefix: Vec<Option<i64>> = vec![None; k as usize];
    prefix[k - 1] = Some(0);
    let mut sum = 0;
    let mut res = i64::MIN;
    for (i, &num) in nums.iter().enumerate() {
        sum += i64::from(num);
        if let Some(prev) = prefix[i % k] {
            res = res.max(sum - prev);
            prefix[i % k] = Some(prev.min(sum));
        } else {
            prefix[i % k] = Some(sum);
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
    fn basics() {}

    #[test]
    fn test() {}
}
