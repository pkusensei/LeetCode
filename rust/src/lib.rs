mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn monotone_increasing_digits(n: i32) -> i32 {
    let s = n.to_string();
    let len = s.len();
    let mut res = vec![];
    let mut prev = 0;
    for (idx, d) in s.bytes().map(|b| i32::from(b - b'0')).enumerate() {
        if prev <= d {
            res.push(d);
            prev = d;
        } else {
            let mut last = prev - 1;
            let mut count = len - idx;
            while let Some(&top) = res.last()
                && top > last
            {
                res.pop();
                last = top - 1;
                count += 1;
            }
            res.push(last);
            res.extend(std::iter::repeat_n(9, count - 1));
            break;
        }
    }
    res.into_iter().fold(0, |acc, d| acc * 10 + d)
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
        assert_eq!(monotone_increasing_digits(10), 9);
        assert_eq!(monotone_increasing_digits(1234), 1234);
        assert_eq!(monotone_increasing_digits(332), 299);
    }

    #[test]
    fn test() {}
}
