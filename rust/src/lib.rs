mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    use std::iter;
    if num * 9 < sum {
        return "".into();
    }
    let mut res = Vec::with_capacity(num as usize);
    let count = (sum / 9) as usize;
    res.extend(iter::repeat_n(b'9', count as usize));
    if sum % 9 > 0 {
        res.push(b'0' + (sum % 9) as u8);
        while res.len() < num as usize {
            res.push(b'0');
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
        assert_eq!(max_sum_of_squares(2, 3), "30");
        assert_eq!(max_sum_of_squares(2, 17), "98");
        assert_eq!(max_sum_of_squares(1, 10), "");
    }

    #[test]
    fn test() {
        assert_eq!(max_sum_of_squares(1, 9), "9");
    }
}
