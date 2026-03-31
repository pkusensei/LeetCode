mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::izip;

pub fn generate_string(str1: &str, str2: &str) -> String {
    let [n1, n2] = [&str1, &str2].map(|s| s.len());
    let mut res = vec![b'#'; n1 + n2 - 1];
    let mut used = vec![false; n1 + n2 - 1];
    for (i, b) in str1.bytes().enumerate() {
        if b == b'T' {
            if izip!(&res[i..], str2.bytes()).any(|(&br, b2)| br != b'#' && br != b2) {
                return "".to_string();
            }
            res[i..i + n2].copy_from_slice(str2.as_bytes());
            used[i..i + n2].fill(true);
        }
    }
    for b in res.iter_mut() {
        if *b == b'#' {
            *b = b'a';
        }
    }
    'out: for (i, b) in str1.bytes().enumerate() {
        if b == b'F' && &res[i..i + n2] == str2.as_bytes() {
            for free in (i..i + n2).rev() {
                if !used[free] {
                    used[free] = true;
                    res[free] = b'b';
                    continue 'out; // skip early return
                }
            }
            return "".to_string();
        }
    }
    String::from_utf8(res).unwrap()
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
        assert_eq!(generate_string("TTFFT", "fff"), "");
    }

    #[test]
    fn test() {}
}
