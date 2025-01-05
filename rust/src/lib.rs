mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shifting_letters(s: String, shifts: &[[i32; 3]]) -> String {
    let (n, mut s) = (s.len(), s.into_bytes());
    let mut prefix = vec![0; n];
    for shift in shifts.iter() {
        let [a, b] = [0, 1].map(|v| shift[v] as usize);
        let c = if shift[2] == 1 { 1 } else { -1 };
        for v in prefix[a..=b].iter_mut() {
            *v += c;
        }
    }
    for (byte, diff) in s.iter_mut().zip(prefix) {
        let num = i32::from(*byte - b'a') + diff;
        *byte = num.rem_euclid(26) as u8 + b'a';
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            shifting_letters("abc".into(), &[[0, 1, 0], [1, 2, 1], [0, 2, 1]]),
            "ace",
        );
        assert_eq!(
            shifting_letters("dztz".into(), &[[0, 0, 0], [1, 1, 1]]),
            "catz"
        );
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
