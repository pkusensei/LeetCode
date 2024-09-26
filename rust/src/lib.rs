mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let is_negative = num.is_negative();
    let mut num = num.abs();
    let mut res = vec![];
    while num > 0 {
        res.push((num % 7) as u8 + b'0');
        num /= 7;
    }
    if is_negative {
        res.push(b'-');
    }
    res.into_iter().rev().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(convert_to_base7(100), "202");
        debug_assert_eq!(convert_to_base7(-7), "-10");
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
