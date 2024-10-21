mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotated_digits(n: i32) -> i32 {
    (1..=n).filter(|&v| check(v)).count() as _
}

const fn check(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let mut rotated = 0;
    let mut factor = 1;
    let mut temp = n;
    while temp > 0 {
        let d = temp % 10;
        let x = match d {
            0 | 1 | 8 => d,
            2 => 5,
            5 => 2,
            6 => 9,
            9 => 6,
            _ => return false,
        };
        rotated += factor * x;
        factor *= 10;
        temp /= 10;
    }
    rotated != n
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(rotated_digits(10), 4);
        debug_assert_eq!(rotated_digits(1), 0);
        debug_assert_eq!(rotated_digits(2), 1);
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
