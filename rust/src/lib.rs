mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut res = HashSet::<i32>::new();
    let mut prev = HashSet::new();
    for &num in arr.iter() {
        // All subarrs ending at `num`
        // Start with single-element subarr [num]
        let mut curr = HashSet::from([num]);
        for v in prev {
            curr.insert(v | num); // BitOr on previous subarrs
        }
        res.extend(&curr);
        prev = curr;
    }
    res.len() as _
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
        assert_eq!(subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
    }

    #[test]
    fn test() {}
}
