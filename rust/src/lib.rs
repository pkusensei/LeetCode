mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_only_letters(s: String) -> String {
    let mut s = s.into_bytes();
    let (mut left, mut right) = (0, s.len() - 1);
    while left < right {
        while left < right && !s[left].is_ascii_alphabetic() {
            left += 1;
        }
        while left < right && !s[right].is_ascii_alphabetic() {
            right -= 1;
        }
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reverse_only_letters("ab-cd".into()), "dc-ba");
        debug_assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".into()),
            "j-Ih-gfE-dCba"
        );
        debug_assert_eq!(
            reverse_only_letters("Test1ng-Leet=code-Q!".into()),
            "Qedo1ct-eeLg=ntse-T!"
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
