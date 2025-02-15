mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromic_subsequence(s: &str) -> i32 {
    let mut prefix = [None; 26];
    let mut suffix = [0; 26];
    for (idx, b) in s.bytes().enumerate() {
        let curr = usize::from(b - b'a');
        prefix[curr].get_or_insert(idx);
        suffix[curr] = idx;
    }
    let mut res = 0;
    for (left, right) in prefix
        .into_iter()
        .zip(suffix)
        .filter_map(|(left, right)| left.map(|v| (v, right)))
    {
        if left < right {
            res += s[1 + left..right]
                .bytes()
                .collect::<std::collections::HashSet<_>>()
                .len();
        }
    }
    res as _
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
        assert_eq!(count_palindromic_subsequence("aabca"), 3);
        assert_eq!(count_palindromic_subsequence("adc"), 0);
        assert_eq!(count_palindromic_subsequence("bbcbaba"), 4);
    }

    #[test]
    fn test() {}
}
