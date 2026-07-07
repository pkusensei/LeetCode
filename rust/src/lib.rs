mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// 0  ->  1
// 11 -> 00
pub fn min_operations(s1: String, s2: String) -> i32 {
    let n = s1.len();
    if s1 == "1" && s2 == "0" {
        return -1;
    }
    let mut s1 = s1.into_bytes();
    let s2 = s2.as_bytes();
    let mut res = 0;
    for i in 0..n {
        if s1[i] == b'0' {
            if s2[i] == b'1' {
                res += 1;
            }
        } else {
            // s1[i] == b'1'
            if s2[i] == b'0' {
                if let Some(b) = s1.get_mut(1 + i)
                    && *b == b'1'
                {
                    *b = b'0';
                    res += 1;
                } else {
                    res += 2; // flip previous idx
                }
            }
        }
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
