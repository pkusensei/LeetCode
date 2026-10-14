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
    use std::iter::repeat_n;
    if 9 * num < sum {
        return "".to_string();
    }
    let mut sum = sum;
    let cap = num as usize;
    let mut res = Vec::with_capacity(cap);
    while sum > 0 {
        let d = 9.min(sum);
        sum -= d;
        res.push(d as u8 + b'0');
    }
    let n = res.len();
    if n < cap {
        res.extend(repeat_n(b'0', cap - n));
    }
    String::from_utf8(res).unwrap()
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
