mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let freq = word2.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let (s, n) = (word1.as_bytes(), word1.len());
    let mut left = 0;
    let mut curr = [0; 26];
    let mut invalid = 0;
    for (right, &b) in s.iter().enumerate() {
        curr[usize::from(b - b'a')] += 1;
        while curr.iter().zip(freq).all(|(&a, b)| a >= b) {
            curr[usize::from(s[left] - b'a')] -= 1;
            left += 1;
        }
        invalid += right + 1 - left;
    }
    (n * (1 + n) / 2 - invalid) as i64
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
        assert_eq!(valid_substring_count("bcca", "abc"), 1);
        assert_eq!(valid_substring_count("abcabc", "abc"), 10);
        assert_eq!(valid_substring_count("abcabc", "aaabc"), 0);
    }

    #[test]
    fn test() {}
}
