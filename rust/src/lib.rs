mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut res = 0;
    for &num in &nums {
        let mut curr = HashMap::new();
        *curr.entry(num).or_insert(0) += 1;
        for (val, c) in map {
            let and = val & num;
            *curr.entry(and).or_insert(0) += c;
        }
        res += curr.get(&k).unwrap_or(&0);
        map = curr;
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
        assert_eq!(count_subarrays(vec![1, 1, 1], 1), 6);
        assert_eq!(count_subarrays(vec![1, 1, 2], 1), 3);
        assert_eq!(count_subarrays(vec![1, 2, 3], 2), 2);
    }

    #[test]
    fn test() {
        assert_eq!(count_subarrays(vec![85, 14, 26, 17, 86, 94], 14), 1);
    }
}
