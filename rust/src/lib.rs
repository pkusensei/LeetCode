mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn special_triplets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut left = HashMap::new();
    let mut right = nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });
    let mut res = 0_i64;
    for &num in nums.iter() {
        right.entry(num).and_modify(|v| *v -= 1);
        let [a, b] = [&left, &right].map(|m| m.get(&(2 * num)).unwrap_or(&0));
        res = (res + a * b) % 1_000_000_007;
        *left.entry(num).or_insert(0) += 1;
    }
    res as i32
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
