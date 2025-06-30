mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn special_triplets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut m1 = HashMap::new();
    let mut m2 = nums.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0i64) += 1;
        acc
    });
    let mut res = 0;
    for &num in nums.iter() {
        m2.entry(num).and_modify(|v| *v -= 1);
        let a = m1.get(&(2 * num)).unwrap_or(&0);
        let b = m2.get(&(2 * num)).unwrap_or(&0);
        res = (res + a * b) % 1_000_000_007;
        *m1.entry(num).or_insert(0i64) += 1;
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
