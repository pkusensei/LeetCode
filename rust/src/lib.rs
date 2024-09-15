mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn to_hex(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    const B: &[u8] = b"0123456789abcdef";
    let mut num = num as u32;
    let mut res = vec![];
    while num > 0 {
        let i = (num % 16) as usize;
        res.push(B[i]);
        num /= 16;
    }
    res.into_iter().rev().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(to_hex(26), "1a");
        debug_assert_eq!(to_hex(-1), "ffffffff");
    }

    #[test]
    fn test() {
        debug_assert_eq!(to_hex(13), "d");
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
