mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
    use std::collections::HashMap;
    const M: i64 = 1_000_000_007;
    let mut sums = HashMap::<i64, i64>::new();
    let mut freqs = HashMap::<i64, i64>::new();
    for &num in nums.iter() {
        let num = i64::from(num);
        let a = num - 1;
        let b = num + 1;
        let freq = 1 + freqs.get(&a).unwrap_or(&0) + freqs.get(&b).unwrap_or(&0);
        let v = freqs.entry(num).or_insert(0);
        *v = (*v + freq) % M;
        let sum = (sums.get(&a).unwrap_or(&0) + sums.get(&b).unwrap_or(&0) + freq * num) % M;
        let v = sums.entry(num).or_insert(0);
        *v = (*v + sum) % M;
    }
    sums.into_values().fold(0, |acc, v| (acc + v) % M) as i32
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
        assert_eq!(sum_of_good_subsequences(&[1, 2, 1]), 14);
        assert_eq!(sum_of_good_subsequences(&[3, 4, 5]), 40);
    }

    #[test]
    fn test() {}
}
