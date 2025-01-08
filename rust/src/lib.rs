mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_string(s: &str) -> String {
    let mut n = s.len();
    let mut count = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = vec![];
    for i in (0..26).chain((0..26).rev()).cycle() {
        if count[i] > 0 {
            res.push(i);
            count[i] -= 1;
            n -= 1;
        }
        if n == 0 {
            break;
        }
    }
    res.into_iter()
        .map(|v| char::from(v as u8 + b'a'))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(sort_string("aaaabbbbcccc"), "abccbaabccba");
        assert_eq!(sort_string("rat"), "art");
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
