mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_substrings(word: &str, k: i32) -> i64 {
    at_least(word.as_bytes(), k) - at_least(word.as_bytes(), 1 + k)
}

fn at_least(s: &[u8], k: i32) -> i64 {
    let n = s.len();
    let mut res = 0;
    let mut left = 0;
    let mut freq = [0; 5];
    let mut cons = 0;
    for (right, &b) in s.iter().enumerate() {
        if let Some(i) = find(b) {
            freq[i] += 1;
        } else {
            cons += 1;
        }
        while freq.iter().all(|&v| v >= 1) && cons >= k {
            res += n - right;
            if let Some(i) = find(s[left]) {
                freq[i] -= 1;
            } else {
                cons -= 1;
            }
            left += 1;
        }
    }
    res as _
}

fn find(b: u8) -> Option<usize> {
    b"aeiou".iter().position(|&v| v == b)
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
    fn test() {
        assert_eq!(count_of_substrings("iqeaouqi", 2), 3)
    }
}
