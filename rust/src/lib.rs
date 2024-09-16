mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn add_strings(num1: &str, num2: &str) -> String {
    let [mut num1, mut num2] =
        [num1, num2].map(|s| s.bytes().rev().map(|b| b - b'0').collect::<Vec<_>>());
    while num1.len() < num2.len() {
        num1.push(0);
    }
    while num2.len() < num1.len() {
        num2.push(0);
    }
    let mut res = vec![];
    let mut carry = 0;
    for (a, b) in num1.into_iter().zip(num2) {
        let c = (a + b + carry) % 10;
        carry = (a + b + carry) / 10;
        res.push(c);
    }
    if carry > 0 {
        res.push(carry);
    }
    res.into_iter()
        .rev()
        .map(|b| char::from(b + b'0'))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(add_strings("11", "123"), "134");
        debug_assert_eq!(add_strings("456", "77"), "533");
        debug_assert_eq!(add_strings("0", "0"), "0");
    }

    #[test]
    fn test() {}

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
