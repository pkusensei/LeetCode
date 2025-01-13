mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reformat(s: String) -> String {
    let [mut letters, mut digits] = [0, 1].map(|_| vec![]);
    for b in s.bytes() {
        if b.is_ascii_alphabetic() {
            letters.push(b);
        } else {
            digits.push(b);
        }
    }
    let [a, b] = match (letters.len() as i16) - (digits.len() as i16) {
        0 | 1 => [letters, digits],
        -1 => [digits, letters],
        _ => return "".to_string(),
    };
    let mut res = Vec::with_capacity(s.len());
    let mut idx = 0;
    while idx < b.len() {
        res.push(a[idx]);
        res.push(b[idx]);
        idx += 1;
    }
    if let Some(&b) = a.get(idx) {
        res.push(b);
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
