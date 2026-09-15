mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_palindromes(s: String, k: i32) -> i32 {
    let (n, s) = (s.len(), s.as_bytes());
    if k == 1 {
        return n as i32;
    }
    let k = k as usize;
    let mut left = 0;
    let mut res = 0;
    while left < n {
        let right = left + k;
        if right <= n && is_palindrome(&s[left..right]) {
            res += 1;
            left = right;
            continue;
        }
        let right = left + k + 1;
        if right <= n && is_palindrome(&s[left..right]) {
            res += 1;
            left = right;
            continue;
        }
        left += 1;
    }
    res
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
