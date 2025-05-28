mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    let mut sum = 0;
    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
        sum += num;
    }
    let mut res = i32::MIN;
    for &num in &nums {
        let curr = sum - num;
        if curr & 1 == 1 {
            continue;
        }
        let cand = curr / 2;
        if (cand == num && freq.get(&cand).is_some_and(|&v| v >= 2))
            || (cand != num && freq.get(&cand).is_some_and(|&v| v > 0))
        {
            res = res.max(num);
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
    fn basics() {}

    #[test]
    fn test() {}
}
