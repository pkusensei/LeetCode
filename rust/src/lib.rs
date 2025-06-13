mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_string(str1: &str, str2: &str) -> String {
    let (n1, n2) = (str1.len(), str2.len());
    let mut res = vec![b'#'; n1 + n2 - 1];
    let mut free = vec![true; n1 + n2 - 1];
    for (idx, b1) in str1.bytes().enumerate() {
        if b1 == b'T' {
            if str2
                .bytes()
                .enumerate()
                .any(|(i2, b2)| res[idx + i2] != b'#' && res[idx + i2] != b2)
            {
                return "".into();
            }
            res[idx..idx + n2].copy_from_slice(str2.as_bytes());
            free[idx..idx + n2].fill(false);
        }
    }
    for b in res.iter_mut().filter(|b| **b == b'#') {
        *b = b'a';
    }
    for (idx, b1) in str1.bytes().enumerate() {
        if b1 == b'F' && str2.bytes().enumerate().all(|(i2, b2)| res[idx + i2] == b2) {
            // Found 'F' and a totally matched str2
            // Seek rightmost free slot
            let Some(i2) = (0..n2).rfind(|&i| free[idx + i]) else {
                return "".into();
            };
            res[idx + i2] = b'b';
            free[idx + i2] = false;
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
        assert_eq!(generate_string("TFTF", "ab"), "ababa");
        assert_eq!(generate_string("TFTF", "abc"), "");
        assert_eq!(generate_string("F", "d"), "a");
    }

    #[test]
    fn test() {}
}
