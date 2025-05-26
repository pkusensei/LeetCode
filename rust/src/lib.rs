mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_frequency(nums: &[i32], k: i32, num_operations: i32) -> i32 {
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    let mut freq = HashMap::new();
    let mut line = BTreeMap::new();
    let mut candids = BTreeSet::new();
    for &num in nums.iter() {
        *freq.entry(num).or_insert(0) += 1;
        *line.entry(num - k).or_insert(0) += 1;
        *line.entry(num + k + 1).or_insert(0) -= 1;
        candids.extend([num, num - k, num + k + 1]);
    }
    let mut prefix = 0;
    let mut res = 1;
    for num in candids {
        let f = *freq.get(&num).unwrap_or(&0);
        prefix += line.get(&num).unwrap_or(&0);
        let change = num_operations.min(prefix - f);
        res = res.max(f + change)
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
        assert_eq!(max_frequency(&[1, 4, 5], 1, 2), 2);
        assert_eq!(max_frequency(&[5, 11, 20, 20], 5, 1), 2);
    }

    #[test]
    fn test() {}
}
