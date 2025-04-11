mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_k_subsequences_with_max_beauty(s: &str, k: i32) -> i32 {
    use itertools::Itertools;
    use std::cmp::Reverse;
    const MOD: usize = 1_000_000_007;
    let count = s
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .filter(|&v| v > 0)
        .sorted_unstable_by_key(|&v| Reverse(v))
        .collect_vec();
    let n = count.len();
    let mut k = k as usize;
    if !(1..=n).contains(&k) {
        return 0;
    }
    let mut freq = count.as_slice();
    let mut res = 1;
    while let Some(&f) = freq.get(0) {
        let mut repeat = freq.iter().filter(|&&v| v == f).count();
        if repeat > k {
            let m = repeat - k;
            let comb = (1..=k).fold(1, |acc, v| acc * (m + v) / v);
            res = (res * comb) % MOD;
            repeat = k;
        }
        for _ in 0..repeat {
            res = res * f % MOD;
        }
        if repeat >= k {
            break;
        }
        freq = &freq[repeat..];
        k -= repeat;
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
        assert_eq!(count_k_subsequences_with_max_beauty("bcca", 2), 4);
        assert_eq!(count_k_subsequences_with_max_beauty("abbcd", 4), 2);
    }

    #[test]
    fn test() {}
}
