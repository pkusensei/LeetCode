mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let n = nums.len();
    if n == 1 {
        return 1;
    }
    let mut doubles = HashSet::new();
    for (i, a) in nums.iter().enumerate() {
        for b in nums.iter().skip(1 + i) {
            doubles.insert(a ^ b);
        }
    }
    let mut triples = HashSet::new();
    for &num in &nums {
        for d in &doubles {
            triples.insert(d ^ num);
        }
    }
    triples.len() as i32
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
