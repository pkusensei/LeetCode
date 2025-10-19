mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lex_greater_permutation(s: &str, target: &str) -> String {
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    backtrack(&mut freq, target.as_bytes(), &mut vec![], false)
        .and_then(|v| String::from_utf8(v).ok())
        .unwrap_or_default()
}

fn backtrack(
    freq: &mut [i16; 26],
    target: &[u8],
    curr: &mut Vec<u8>,
    inc: bool,
) -> Option<Vec<u8>> {
    if inc {
        for (i, &v) in freq.iter().enumerate() {
            if v > 0 {
                curr.extend(std::iter::repeat_n(i as u8 + b'a', v as usize));
            }
        }
        return Some(curr.clone());
    }
    if target.is_empty() {
        return None;
    }
    let idx = usize::from(target[0] - b'a');
    for i in idx..26 {
        if freq[i] > 0 {
            freq[i] -= 1;
            curr.push(i as u8 + b'a');
            if let Some(v) = backtrack(freq, &target[1..], curr, i > idx) {
                return Some(v);
            }
            freq[i] += 1;
            curr.pop();
        }
    }
    return None;
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
        assert_eq!(lex_greater_permutation("leet", "code"), "eelt");
        assert_eq!(lex_greater_permutation("baba", "bbaa"), "");
    }

    #[test]
    fn test() {}
}
