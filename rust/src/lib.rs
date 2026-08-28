mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lex_palindromic_permutation(s: &str, target: &str) -> String {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut single = None;
    for (i, f) in freq.iter().enumerate() {
        if f & 1 == 1 {
            if single.is_some() {
                return "".to_string();
            }
            single = Some(i as u8 + b'a');
        }
    }
    let mut half = freq.map(|v| v / 2);
    if let Some(v) = dfs(&mut half, 0, false, single, target.as_bytes(), &mut vec![]) {
        String::from_utf8(v).unwrap()
    } else {
        "".to_string()
    }
}

fn dfs(
    freq: &mut [i32],
    idx: usize,
    bigger: bool,
    single: Option<u8>,
    target: &[u8],
    curr: &mut Vec<u8>,
) -> Option<Vec<u8>> {
    use std::iter::repeat_n;

    let n = target.len();
    if bigger || idx >= n / 2 {
        let mut temp = curr.clone();
        if idx < n / 2 {
            for (i, &f) in freq.iter().enumerate() {
                temp.extend(repeat_n(i as u8 + b'a', f as usize));
            }
        }
        if let Some(v) = single {
            temp.push(v);
        }
        temp.extend_from_within(..n / 2);
        temp[(1 + n) / 2..].reverse();
        return if bigger || temp.as_slice() > target {
            Some(temp)
        } else {
            None
        };
    }
    let low = usize::from(target[idx] - b'a');
    for i in low..26 {
        if freq[i] > 0 {
            freq[i] -= 1;
            curr.push(i as u8 + b'a');
            if let Some(v) = dfs(freq, 1 + idx, i > low, single, target, curr) {
                return Some(v);
            }
            curr.pop();
            freq[i] += 1;
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
    fn basics() {
        assert_eq!(lex_palindromic_permutation("baba", "abba"), "baab")
    }

    #[test]
    fn test() {}
}
