mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::chain;
use std::iter;

pub fn smallest_number(num: &str, t: i64) -> String {
    if !check(t) {
        return "-1".into();
    }
    let n = num.len();
    let mut prefix = vec![t; 1 + n];
    for (idx, b) in num.bytes().enumerate() {
        if b == b'0' {
            break;
        }
        // How much of `t` is still needed at idx
        prefix[1 + idx] = prefix[idx] / gcd(prefix[idx], (b - b'0').into());
    }
    if prefix.last().is_some_and(|&v| v == 1) {
        return num.into();
    }
    // zero must be incremented
    // everything after could be freely chosen to meet the requirements
    let zero = num.find('0').unwrap_or(n - 1);
    for idx in (0..=zero).rev() {
        let req = prefix[idx];
        let digits = n - 1 - idx;
        let start = 1 + i64::from(num.as_bytes()[idx] - b'0');
        for d in start..=9 {
            let end = fill(req / gcd(req, d), digits);
            if end.len() <= digits {
                return chain!(num.bytes().take(idx), iter::once(d as u8 + b'0'), end)
                    .map(char::from)
                    .collect();
            }
        }
    }
    String::from_utf8(fill(t, 1 + n)).unwrap()
}

fn fill(mut req: i64, len: usize) -> Vec<u8> {
    let mut res = Vec::with_capacity(len);
    for d in (2..=9).rev() {
        while req % d == 0 {
            res.push(d as u8 + b'0');
            req /= d;
        }
    }
    res.extend(iter::repeat_n(b'1', len.saturating_sub(res.len())));
    res.reverse();
    res
}

fn check(mut t: i64) -> bool {
    for p in [2, 3, 5, 7] {
        while t > 1 && t % p == 0 {
            t /= p;
        }
    }
    t == 1
}

const fn gcd(a: i64, b: i64) -> i64 {
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
        assert_eq!(smallest_number("1234", 256), "1488");
        assert_eq!(smallest_number("12355", 50), "12355");
        assert_eq!(smallest_number("11111", 26), "-1");
    }
    #[test]
    fn test() {}
}
