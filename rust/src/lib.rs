mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromic_subsequence(s: &str) -> i32 {
    let n = s.len();
    let mut pre_masks = Vec::with_capacity(n);
    for b in s.bytes() {
        let pos = usize::from(b - b'a');
        pre_masks.push((1_i32 << pos) | pre_masks.last().unwrap_or(&0));
    }
    let mut suf_masks = Vec::with_capacity(n);
    for b in s.bytes().rev() {
        suf_masks.push((1 << (b - b'a')) | suf_masks.last().unwrap_or(&0));
    }
    suf_masks.reverse();
    let mut res = [0; 26];
    for i in 1..n - 1 {
        let b = s.as_bytes()[i];
        res[usize::from(b - b'a')] |= pre_masks[i - 1] & suf_masks[1 + i];
    }
    res.iter().map(|v| v.count_ones() as i32).sum()
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
        assert_eq!(count_palindromic_subsequence("bbcbaba"), 4);
    }

    #[test]
    fn test() {}
}
