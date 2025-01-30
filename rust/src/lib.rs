mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reformat_number(number: String) -> String {
    let mut s = number.into_bytes();
    s.retain(|b| b.is_ascii_digit());
    let mut curr = s.as_slice();
    let mut res = vec![];
    while curr.len() > 4 {
        let (left, right) = curr.split_at(3);
        res.extend_from_slice(left);
        res.push(b'-');
        curr = right;
    }
    match curr.len() {
        0 => (),
        4 => {
            res.extend_from_slice(&curr[..2]);
            res.push(b'-');
            res.extend_from_slice(&curr[2..]);
        }
        _ => {
            res.extend_from_slice(curr);
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
