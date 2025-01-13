mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_happy_string(n: i32, mut k: i32) -> String {
    let n = n as usize;
    let mut res = Vec::with_capacity(n);
    backtrack(n, &mut k, &mut res);
    String::from_utf8(res).unwrap()
}

fn backtrack(n: usize, k: &mut i32, res: &mut Vec<u8>) -> bool {
    if res.len() == n {
        *k -= 1;
        return *k == 0;
    }
    for &b in b"abc" {
        if res.last().is_some_and(|&v| v == b) {
            continue;
        }
        res.push(b);
        if backtrack(n, k, res) {
            return true;
        }
        res.pop();
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_happy_string(1, 3), "c");
        assert_eq!(get_happy_string(1, 4), "");
        assert_eq!(get_happy_string(3, 9), "cab");
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

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
