mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_number(num: String, t: i64) -> String {
    if !check(t) {
        return "-1".to_string();
    }
    let (s, n) = (num.as_bytes(), num.len());
    let mut remaining = vec![t; 1 + n];
    let mut zero = n - 1;
    for (idx, &b) in s.iter().enumerate() {
        let v = i64::from(b - b'0');
        if v == 0 {
            zero = idx;
            break;
        }
        remaining[1 + idx] = remaining[idx] / gcd(remaining[idx], v);
    }
    if remaining[n] == 1 {
        return num;
    }
    for idx in (0..=zero).rev() {
        let requirement = remaining[idx];
        let len = n - 1 - idx;
        let start = i64::from(1 + s[idx] - b'0');
        for d in start..=9 {
            let suffix = fill_in(requirement / gcd(requirement, d), len);
            if suffix.len() <= len {
                let mut res = s[..idx].to_vec();
                res.push(d as u8 + b'0');
                res.extend(suffix);
                return String::from_utf8(res).unwrap();
            }
        }
    }
    String::from_utf8(fill_in(t, 1 + n)).unwrap()
}

fn fill_in(mut requirement: i64, len: usize) -> Vec<u8> {
    let mut res = Vec::with_capacity(len);
    for d in (2..=9).rev() {
        while requirement % d == 0 {
            res.push(d as u8 + b'0');
            requirement /= d;
        }
    }
    res.extend(repeat_n(b'1', len.saturating_sub(res.len())));
    res.reverse();
    res
}

fn check(mut t: i64) -> bool {
    for p in [2, 3, 5, 7] {
        while t % p == 0 {
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

    #[allow(unused_imports)]
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
