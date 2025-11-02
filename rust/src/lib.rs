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
    for (idx, f) in freq.iter().enumerate() {
        if f & 1 == 1 {
            if single.is_none() {
                single = Some(idx as u8 + b'a');
            } else {
                return "".to_string();
            }
        }
    }
    let n = s.len();
    let mut candids = freq.map(|v| v / 2);
    let res = build(
        target.as_bytes(),
        single,
        &mut candids,
        &target.as_bytes()[..n / 2],
        true,
        &mut vec![],
    )
    .unwrap_or_default();
    String::from_utf8(res).unwrap_or_default()
}

fn build(
    target: &[u8],
    single: Option<u8>,
    candids: &mut [i32; 26],
    half: &[u8],
    tight: bool,
    curr: &mut Vec<u8>,
) -> Option<Vec<u8>> {
    if half.is_empty() {
        let mut temp = curr.clone();
        if let Some(v) = single {
            temp.push(v);
        }
        temp.extend(curr.iter().rev());
        if temp.as_slice() > target {
            return Some(temp);
        }
        return None;
    }
    let lower = if tight { half[0] } else { b'a' };
    for b in lower..=b'z' {
        let bi = usize::from(b - b'a');
        if candids[bi] > 0 {
            candids[bi] -= 1;
            curr.push(b);
            if let Some(v) = build(
                target,
                single,
                candids,
                &half[1..],
                tight && b == lower,
                curr,
            ) {
                return Some(v);
            }
            curr.pop();
            candids[bi] += 1;
        }
    }
    None
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
        assert_eq!(lex_palindromic_permutation("baba", "abba"), "baab");
        assert_eq!(lex_palindromic_permutation("baba", "bbaa"), "");
        assert_eq!(lex_palindromic_permutation("abc", "abb"), "");
        assert_eq!(lex_palindromic_permutation("aac", "abb"), "aca");
    }

    #[test]
    fn test() {
        assert_eq!(lex_palindromic_permutation("aabb", "aaaa"), "abba");
    }
}
