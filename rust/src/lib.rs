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

pub fn longest_balanced(s: String) -> i32 {
    // uniform
    let mut res = s
        .as_bytes()
        .chunk_by(|a, b| a == b)
        .map(|w| w.len())
        .max()
        .unwrap_or(1);
    let mut seen = HashMap::from([([0, 0], 0)]);
    let mut curr = [0; 3];
    let mut prefix = Vec::from([curr]);
    for (idx, b) in s.bytes().enumerate() {
        curr[usize::from(b - b'a')] += 1;
        prefix.push(curr);
        // [a-b, b-c]
        let diff = [curr[0] - curr[1], curr[1] - curr[2]];
        // +1 to offset seen[[0,0]]=0
        let left = *seen.entry(diff).or_insert(1 + idx);
        res = res.max(1 + idx - left);
    }
    res.max(f2(&prefix, 0, 1))
        .max(f2(&prefix, 0, 2))
        .max(f2(&prefix, 1, 2)) as i32
}

fn f2(prefix: &[[i32; 3]], i0: usize, i1: usize) -> usize {
    let n = prefix.len();
    let i2 = 3 - i0 - i1;
    let mut res = 1;
    let mut discard = 0;
    let mut left = 1;
    while left < n {
        while left < n && prefix[left][i2] > discard {
            discard = prefix[left][i2];
            left += 1;
        }
        let mut right = left;
        let mut seen = HashMap::from([(0, left - 1)]);
        while right < n && prefix[right][i2] == discard {
            let left_f = prefix[left - 1];
            let f = prefix[right];
            let diff = (f[i1] - f[i0]) - (left_f[i1] - left_f[i0]);
            let v = *seen.entry(diff).or_insert(right);
            res = res.max(right - v);
            right += 1;
        }
        left = right;
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
    fn basics() {}

    #[test]
    fn test() {}
}
