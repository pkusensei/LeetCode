mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn subdomain_visits(cpdomains: &[&str]) -> Vec<String> {
    let mut counts = HashMap::new();
    for s in cpdomains.iter() {
        let Some((n, mut domain)) = s.split_once(' ') else {
            continue;
        };
        let Ok(n) = n.parse::<i32>() else {
            continue;
        };
        while !domain.is_empty() {
            *counts.entry(domain).or_insert(0) += n;
            let Some((_, d)) = domain.split_once('.') else {
                break;
            };
            domain = d;
        }
    }
    counts
        .into_iter()
        .map(|(k, v)| format!("{v} {k}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            subdomain_visits(&["9001 discuss.leetcode.com"]),
            ["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"],
        );
        sort_eq(
            subdomain_visits(&[
                "900 google.mail.com",
                "50 yahoo.com",
                "1 intel.mail.com",
                "5 wiki.org",
            ]),
            [
                "901 mail.com",
                "50 yahoo.com",
                "900 google.mail.com",
                "5 wiki.org",
                "5 org",
                "1 intel.mail.com",
                "951 com",
            ],
        );
    }

    #[test]
    fn test() {}

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
}
