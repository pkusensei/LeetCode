mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_active_sections_after_trade(s: String) -> i32 {
    let arr: Vec<_> = s.as_bytes().chunk_by(|a, b| a == b).collect();
    let ones = s.bytes().fold(0, |acc, b| acc + i32::from(b - b'0'));
    let n = arr.len();
    if n < 3 {
        return ones;
    }
    let mut res = 0;
    for i in 1..n - 1 {
        if arr[i][0] == b'1' {
            let d = (arr[i - 1].len() + arr[1 + i].len()) as i32;
            res = res.max(d);
        }
    }
    ones + res
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
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
