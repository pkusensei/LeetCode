mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::BTreeMap;
    let n = nums.len();
    let mut map = BTreeMap::new();
    for &num in nums.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut res = Vec::with_capacity(n);
    while res.len() < n {
        for (k, v) in map.iter_mut() {
            if *v > 0 {
                res.push(*k);
                *v -= 1;
            }
        }
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
