mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_valid_parentheses(s: String) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut pairs = vec![false; n];
    let mut st = vec![];
    for (idx, &b) in s.iter().enumerate() {
        if let Some(&top) = st.last()
            && s[top] == b'('
            && b == b')'
        {
            pairs[top] = true;
            pairs[idx] = true;
            st.pop();
        } else {
            st.push(idx);
        }
    }
    pairs
        .chunk_by(|a, b| *a && *b)
        .filter_map(|w| {
            let v = w.len() as i32;
            if v & 1 == 0 { Some(v) } else { None }
        })
        .max()
        .unwrap_or(0)
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
