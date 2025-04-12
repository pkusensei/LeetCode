mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_integers(n: i32, k: i32) -> i64 {
    use itertools::Itertools;
    use std::collections::HashSet;
    let half = (1 + n) as u32 / 2;
    let min = 10i32.pow(half - 1);
    let max = 10i32.pow(half);
    let mut seen = HashSet::new();
    let mut res = 0;
    for num in min..max {
        let mut a = num.to_string();
        let b: String = a.chars().rev().collect();
        a.push_str(&b[(n & 1) as usize..]);
        if a.parse::<i64>().ok().is_none_or(|v| v % i64::from(k) > 0) {
            continue;
        }
        let sorted = a.into_bytes().into_iter().sorted_unstable().collect_vec();
        if !seen.insert(sorted.clone()) {
            continue;
        }
        let count = sorted.iter().fold([0; 10], |mut acc, &b| {
            acc[usize::from(b - b'0')] += 1;
            acc
        });
        let n = i64::from(n);
        let first = n - count[0];
        let mut perm = first * fact(n - 1);
        for freq in count {
            if freq > 1 {
                perm /= fact(freq);
            }
        }
        res += perm;
    }
    res
}

const fn fact(n: i64) -> i64 {
    if n < 2 { 1 } else { n * fact(n - 1) }
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
        assert_eq!(count_good_integers(3, 5), 27);
        assert_eq!(count_good_integers(1, 4), 2);
        assert_eq!(count_good_integers(5, 6), 2468);
    }

    #[test]
    fn test() {}
}
