mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_unique_emails(emails: &[&str]) -> i32 {
    emails
        .iter()
        .map(|s| process(s))
        .collect::<std::collections::HashSet<_>>()
        .len() as _
}

fn process(s: &str) -> String {
    let (a, b) = s.split_once('@').unwrap_or_default();
    let mut res = vec![];
    for b in a.bytes() {
        match b {
            b'+' => break,
            b'.' => continue,
            _ => res.push(b),
        }
    }
    res.push(b'@');
    res.extend_from_slice(b.as_bytes());
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_unique_emails(&[
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]),
            2
        );
        debug_assert_eq!(
            num_unique_emails(&["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]),
            3
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            num_unique_emails(&[
                "test.email+alex@leetcode.com",
                "test.email.leet+alex@code.com"
            ]),
            2
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
