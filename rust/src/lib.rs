mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn lex_greater_permutation(s: String, target: String) -> String {
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    if let Some(mut v) = dfs(&mut freq, target.as_bytes(), false) {
        v.reverse();
        String::from_utf8(v).unwrap()
    } else {
        "".to_string()
    }
}

fn dfs(freq: &mut [i32; 26], t: &[u8], bigger: bool) -> Option<Vec<u8>> {
    if bigger {
        let mut res = vec![];
        for (i, &f) in freq.iter().enumerate().rev() {
            res.extend(repeat_n(i as u8 + b'a', f as usize));
        }
        return Some(res);
    }
    if t.is_empty() {
        return None;
    }
    let head = usize::from(t[0] - b'a');
    for curr in head..26 {
        if freq[curr] > 0 {
            freq[curr] -= 1;
            if let Some(mut v) = dfs(freq, &t[1..], curr > head) {
                v.push(curr as u8 + b'a');
                return Some(v);
            } else {
                freq[curr] += 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
