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

pub fn longest_balanced(s: &str) -> i32 {
        let s = s.as_bytes();
        let uniform = s
            .chunk_by(|a, b| a == b)
            .map(|ch| ch.len())
            .max()
            .unwrap_or(1);
        // a - freq[0]
        // b - freq[1]
        // c - freq[2]
        let mut curr = [0; 3];
        let mut freq = Vec::from([curr]);
        let mut seen = HashMap::from([((0, 0), 0)]);
        let mut res = 1;
        for (idx, &b) in s.iter().enumerate() {
            curr[usize::from(b - b'a')] += 1;
            freq.push(curr);
            // (freq_b - freq_a, freq_c - freq_a)
            if let Some(&prev) = seen.get(&(curr[1] - curr[0], curr[2] - curr[0])) {
                res = res.max(idx + 1 - prev);
            } else {
                seen.insert((curr[1] - curr[0], curr[2] - curr[0]), 1 + idx);
            }
        }
        res.max(uniform)
            .max(count2(&freq, 0, 1))
            .max(count2(&freq, 1, 2))
            .max(count2(&freq, 0, 2)) as i32
}

fn count2(freq: &[[i32; 3]], i1: usize, i2: usize) -> usize {
    let n = freq.len();
    let i3 = 3 - i1 - i2;
    let mut discard = 0;
    let mut left = 1;
    let mut res = 1;
    while left < n {
        while left < n && freq[left][i3] - discard > 0 {
            discard = freq[left][i3];
            left += 1;
        }
        if left >= n {
            break;
        }
        let mut right = left;
        let mut seen = HashMap::from([(0, left - 1)]);
        while right < n && freq[right][i3] == discard {
            let delta =
                (freq[right][i2] - freq[right][i1]) - (freq[left - 1][i2] - freq[left - 1][i1]);
            if let Some(&prev) = seen.get(&delta) {
                res = res.max(right - prev);
            } else {
                seen.insert(delta, right);
            }
            right += 1;
        }
        left = right; // fast forward
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
        assert_eq!(longest_balanced("aabcc"), 3);
        assert_eq!(longest_balanced("abbac"), 4);
        assert_eq!(longest_balanced("aba"), 2);
    }

    #[test]
    fn test() {
        assert_eq!(longest_balanced("cbbbc"), 3);
        assert_eq!(longest_balanced("abcbc"), 4);
        assert_eq!(longest_balanced("bac"), 3);
    }
}
