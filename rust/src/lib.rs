mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_equal_adjacent_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let noop = nums.windows(2).filter(|w| w[0] == w[1]).count() as i32;
    let mut res = noop;
    let mut val_ids = HashMap::<_, Vec<_>>::new();
    for (idx, &num) in nums.iter().enumerate() {
        val_ids.entry(num).or_default().push(idx);
    }
    for (&target, ids) in val_ids.iter() {
        let mut freq = HashMap::new();
        for &i in ids {
            if i == 0 {
                if nums[1 + i] != target {
                    *freq.entry(nums[1 + i]).or_insert(0) += 1;
                }
            } else if i == n - 1 {
                if nums[i - 1] != target {
                    *freq.entry(nums[i - 1]).or_insert(0) += 1;
                }
            } else {
                if nums[1 + i] != target {
                    *freq.entry(nums[1 + i]).or_insert(0) += 1;
                }
                if nums[i - 1] != target {
                    *freq.entry(nums[i - 1]).or_insert(0) += 1;
                }
            }
        }
        res = res.max(noop + freq.values().max().unwrap_or(&0))
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
    fn test() {}
}
