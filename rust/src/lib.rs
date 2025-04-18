mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_substrings(s: String, k: i32) -> i32 {
    use std::collections::HashSet;
    let vowels = HashSet::from([b'a', b'e', b'i', b'o', b'u']);
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 0;
    for left in 0..n - 1 {
        let [mut vs, mut cs] = [0, 0];
        for right in left..n {
            if vowels.contains(&s[right]) {
                vs += 1
            } else {
                cs += 1
            }
            res += i32::from(right - left > 0 && vs == cs && vs * cs % k == 0);
        }
    }
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
