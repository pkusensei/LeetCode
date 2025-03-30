mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// (i+k)%n
// gcd(n, k) => smallest "unit" of circular subarray
pub fn make_sub_k_sum_equal(arr: &[i32], k: i32) -> i64 {
    let n = arr.len();
    let k = k as usize;
    let b_count = gcd(n, k);
    let mut buckets = arr
        .iter()
        .enumerate()
        .fold(vec![vec![]; b_count], |mut acc, (i, &num)| {
            acc[i % b_count].push(num);
            acc
        });
    let mut res = 0;
    for bucket in buckets.iter_mut() {
        let len = bucket.len();
        let (_, &mut med, _) = bucket.select_nth_unstable(len / 2);
        res += bucket
            .iter()
            .map(|v| i64::from((v - med).abs()))
            .sum::<i64>();
    }
    res
}

const fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(make_sub_k_sum_equal(&[1, 4, 1, 3], 2), 1);
        assert_eq!(make_sub_k_sum_equal(&[2, 5, 5, 7], 3), 5);
    }

    #[test]
    fn test() {}
}
