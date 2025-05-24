mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_substrings(s: String, k: i32) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut freq = [0; 26];
    let mut count = 0;
    let mut left = 0;
    for (right, &b) in s.iter().enumerate() {
        freq[usize::from(b - b'a')] += 1;
        while freq.iter().any(|&v| v >= k) {
            freq[usize::from(s[left] - b'a')] -= 1;
            left += 1;
        }
        count += right + 1 - left;
    }
    (n * (1 + n) / 2 - count) as i32
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
