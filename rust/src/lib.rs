mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_substrings(s: &str) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut left_zero = vec![-1; 1 + n];
    for i in 0..n {
        left_zero[1 + i] = if i == 0 || s[i - 1] == b'0' {
            i as i32
        } else {
            left_zero[i]
        };
    }
    let mut res = 0;
    for (right, &b) in s.iter().enumerate() {
        let mut f0 = if b == b'0' { 1 } else { 0 };
        let mut end = 1 + right as i32;
        while end > 0 && f0 * f0 <= n {
            let f1 = (1 + right) as i32 - left_zero[end as usize] - f0 as i32;
            if (f0 * f0) as i32 <= f1 {
                res += (end as i32 - left_zero[end as usize]).min(1 + f1 - (f0 * f0) as i32);
            }
            end = left_zero[end as usize];
            f0 += 1;
        }
    }
    res as i32
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
        assert_eq!(number_of_substrings("00011"), 5);
        assert_eq!(number_of_substrings("101101"), 16);
    }

    #[test]
    fn test() {}
}
