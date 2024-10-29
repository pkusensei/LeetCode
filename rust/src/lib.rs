mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shifting_letters(s: &str, shifts: &[i32]) -> String {
    let (_, v) = s.bytes().zip(shifts.iter()).rev().fold(
        (0, Vec::with_capacity(s.len())),
        |(num, mut res), (b, &n)| {
            let num = (num + n) % 26;
            res.insert(0, shift(b, num as u8));
            (num, res)
        },
    );
    String::from_utf8(v).unwrap()
}

fn shift(b: u8, num: u8) -> u8 {
    (b - b'a' + num) % 26 + b'a'
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shifting_letters("abc", &[3, 5, 9],), "rpl");
        debug_assert_eq!(shifting_letters("aaa", &[1, 2, 3],), "gfd");
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
}
