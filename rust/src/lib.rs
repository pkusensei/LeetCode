mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_perms_di_sequence(s: String) -> i32 {
    const M: i32 = 1_000_000_007;
    let n = s.len();
    let mut prev = vec![0; 1 + n];
    prev[0] = 1;
    for (i1, b) in s.bytes().enumerate() {
        let mut curr = vec![0; 1 + n];
        let mut prefix = 0;
        if b == b'I' {
            for i2 in 0..=1 + i1 {
                if i2 > 0 {
                    prefix = (prefix + prev[i2 - 1]) % M;
                }
                curr[i2] = prefix;
            }
        } else {
            for i2 in (0..1 + i1).rev() {
                if i2 <= i1 {
                    prefix = (prefix + prev[i2]) % M;
                }
                curr[i2] = prefix;
            }
        }
        prev = curr;
    }
    prev.iter().fold(0, |acc, v| (acc + v) % M)
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
