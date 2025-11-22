mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn total_waviness(num1: i64, num2: i64) -> i64 {
    let [s1, s2] = [num1 - 1, num2].map(|v| v.to_string().into_bytes());
    let a = dfs(&s1, true, false, b'0', b'0', &mut HashMap::new())[0];
    let b = dfs(&s2, true, false, b'0', b'0', &mut HashMap::new())[0];
    b - a
}

// [waviness, count of numbers]
fn dfs(
    s: &[u8],
    tight: bool,
    start: bool,
    left: u8,
    mid: u8,
    memo: &mut HashMap<(usize, bool, bool, u8, u8), [i64; 2]>,
) -> [i64; 2] {
    if s.is_empty() {
        return [0, 1]; // [no more waviness, one number]
    }
    let k = (s.len(), tight, start, left, mid);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let upper = if tight { s[0] } else { b'9' };
    let mut res = [0; 2];
    for digit in b'0'..=upper {
        let ntight = tight && digit == upper;
        let nstart = start || mid > b'0';
        let [sub_wav, sub_count] = dfs(&s[1..], ntight, nstart, mid, digit, memo);
        res[0] += sub_wav;
        res[1] += sub_count;
        if start && nstart {
            // This peak/valey propagates to all sub numbers
            if left > mid && digit > mid {
                res[0] += sub_count;
            }
            if left < mid && digit < mid {
                res[0] += sub_count;
            }
        }
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
    fn basics() {
        assert_eq!(total_waviness(4848, 4848), 2);
        assert_eq!(total_waviness(120, 130), 3);
        assert_eq!(total_waviness(198, 202), 3);
    }

    #[test]
    fn test() {}
}
