mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_concatenated_length(words: &[&str]) -> i32 {
    let s = words[0].as_bytes();
    let n = s.len();
    n as i32 + dfs(words, 1, s[0], s[n - 1], &mut HashMap::new())
}

fn dfs(
    words: &[&str],
    idx: usize,
    first: u8,
    last: u8,
    memo: &mut HashMap<(usize, u8, u8), i32>,
) -> i32 {
    if idx >= words.len() {
        return 0;
    }
    if let Some(&v) = memo.get(&(idx, first, last)) {
        return v;
    }
    let s = words[idx].as_bytes();
    let n = s.len();
    let prepend = n as i32 - i32::from(s[n - 1] == first) + dfs(words, 1 + idx, s[0], last, memo);
    let append = n as i32 - i32::from(s[0] == last) + dfs(words, 1 + idx, first, s[n - 1], memo);
    let res = prepend.min(append);
    memo.insert((idx, first, last), res);
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
    fn basics() {
        assert_eq!(minimize_concatenated_length(&["aa", "ab", "bc"]), 4);
        assert_eq!(minimize_concatenated_length(&["ab", "b"]), 2);
        assert_eq!(minimize_concatenated_length(&["aaa", "c", "aba"]), 6);
    }

    #[test]
    fn test() {
        assert_eq!(minimize_concatenated_length(&["a", "cba", "a"]), 3);
    }
}
