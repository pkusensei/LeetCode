mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut freq = [0; 26];
    let mut left = 0;
    let mut res = 1;
    for (right, &b) in s.iter().enumerate() {
        freq[usize::from(b - b'a')] += 1;
        while freq[usize::from(b - b'a')] > 2 {
            freq[usize::from(s[left] - b'a')] -= 1;
            left += 1;
        }
        res = res.max(1 + right - left)
    }
    res as i32
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
