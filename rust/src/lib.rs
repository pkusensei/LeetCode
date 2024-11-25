mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

const P: i64 = 127;
const MOD: i64 = 1_000_000_007;

pub fn longest_dup_substring(s: &str) -> String {
    let n = s.len();
    let [mut prefix, mut pows] = [0, 1].map(|_| Vec::with_capacity(n));
    prefix.push(0);
    pows.push(1);
    for b in s.bytes() {
        prefix.push((prefix.last().unwrap_or(&0) * P + i64::from(b)) % MOD);
        pows.push((pows.last().unwrap_or(&1) * P) % MOD);
    }

    let (mut left, mut right) = (0, n);
    let mut res = "";
    while left < right {
        let mid = left + (right - left) / 2;
        if let Some(v) = check(s, mid, &prefix, pows[mid]) {
            if v.len() > res.len() {
                res = v;
            }
            left = mid + 1
        } else {
            right = mid
        }
    }
    res.to_string()
}

fn check<'a>(s: &'a str, mid: usize, prefix: &[i64], pow: i64) -> Option<&'a str> {
    let n = s.len();
    let mut map = HashMap::new();
    for i1 in 0..=n - mid {
        let a = (prefix[i1 + mid] - prefix[i1] * pow).rem_euclid(MOD);
        if let Some(&i2) = map.get(&a) {
            if s[i1..i1 + mid] == s[i2..i2 + mid] {
                return Some(&s[i1..i1 + mid]);
            }
        }
        map.insert(a, i1);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_dup_substring("banana"), "ana");
        debug_assert_eq!(longest_dup_substring("abcd"), "");
    }

    #[test]
    fn test() {
        debug_assert_eq!(longest_dup_substring("aa"), "a");
        debug_assert_eq!(
            longest_dup_substring("nnpxouomcofdjuujloanjimymadkuepightrfodmauhrsy"),
            "ma"
        );
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
