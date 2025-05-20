mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut k = i128::from(k);
    let n = operations.len();
    let mut len = 1_i128 << n;
    let mut count = 0;
    for &op in operations.iter().rev() {
        len >>= 1;
        if k > len {
            count += op;
            k -= len;
        }
    }
    char::from(b'a' + (count % 26) as u8)
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(kth_character(10, &[0, 1, 0, 1]), 'b');
    }

    #[test]
    fn test() {}
}
