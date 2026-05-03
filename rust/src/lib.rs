mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && search(format!("{s}{s}").as_bytes(), goal.as_bytes())
}

fn search(hay: &[u8], needle: &[u8]) -> bool {
    let lps = compute_lps(needle);
    let mut len = 0;
    for &val in hay {
        while len > 0 && needle[len] != val {
            len = lps[len - 1]
        }
        if needle.get(len).is_some_and(|&b| b == val) {
            len += 1;
        }
        if len == needle.len() {
            break;
        }
    }
    len == needle.len()
}

fn compute_lps(needle: &[u8]) -> Vec<usize> {
    let n = needle.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && needle[idx] != needle[len] {
            len = lps[len - 1];
        }
        if needle[idx] == needle[len] {
            len += 1
        }
        lps[idx] = len;
    }
    lps
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
        assert_eq!(compute_lps(b"ababb"), [0, 0, 1, 2, 0]);
        assert_eq!(compute_lps(b"ababc"), [0, 0, 1, 2, 0]);
        assert_eq!(compute_lps(b"abab"), [0, 0, 1, 2])
    }

    #[test]
    fn test() {}
}
