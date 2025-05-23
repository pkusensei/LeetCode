mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_winning_sequences(s: &str) -> i32 {
    dfs(s.as_bytes(), 0, 0, 0, &mut HashMap::new())
}

fn dfs(
    s: &[u8],
    idx: usize,
    prev: u8,
    score: i32,
    memo: &mut HashMap<(usize, u8, i32), i32>,
) -> i32 {
    if idx >= s.len() {
        return i32::from(score < 0);
    }
    let k = (idx, prev, score);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let mut res = 0;
    for b in [b'E', b'F', b'W'].into_iter().filter(|&v| v != prev) {
        let new_score = score
            + match [s[idx], b] {
                [b'F', b'E'] | [b'W', b'F'] | [b'E', b'W'] => 1,
                [b'E', b'F'] | [b'F', b'W'] | [b'W', b'E'] => -1,
                _ => 0,
            };
        res += dfs(s, 1 + idx, b, new_score, memo);
        res %= 1_000_000_007;
    }
    memo.insert(k, res);
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
        assert_eq!(count_winning_sequences("FFF"), 3);
        assert_eq!(count_winning_sequences("FWEFW"), 18);
    }

    #[test]
    fn test() {}
}
