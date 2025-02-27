mod dsu;
mod helper;
mod trie;

use std::{
    iter::{once, repeat},
    str,
};

#[allow(unused_imports)]
use helper::*;

pub fn k_mirror(k: i32, n: i32) -> i64 {
    let mut res = 0;
    let mut s = vec![b'0'];
    for _ in 0..n {
        loop {
            s = gen_mirror_k(k as u8 + b'0', s);
            let Some(num) = str::from_utf8(&s)
                .ok()
                .and_then(|s| i64::from_str_radix(s, k as u32).ok())
            else {
                continue;
            };
            if is_palindrome(num.to_string().bytes()) {
                res += num;
                break;
            }
        }
    }
    res
}

fn gen_mirror_k(k: u8, mut s: Vec<u8>) -> Vec<u8> {
    let n = s.len();
    let mid = s.len() / 2;
    for idx in mid..s.len() {
        if s[idx] + 1 < k {
            let d = 1 + s[idx];
            s[idx] = d;
            s[n - 1 - idx] = d;
            for i in mid..idx {
                s[i] = b'0';
                s[n - 1 - i] = b'0';
            }
            return s;
        }
    }
    once(b'1')
        .chain(repeat(b'0').take(n - 1))
        .chain(once(b'1'))
        .collect()
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
        assert_eq!(k_mirror(2, 5), 25);
        assert_eq!(k_mirror(3, 7), 499);
        assert_eq!(k_mirror(7, 17), 20379000);
    }

    #[test]
    fn test() {}
}
