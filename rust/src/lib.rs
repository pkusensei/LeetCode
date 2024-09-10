mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn integer_break(n: i32) -> i32 {
    match n {
        ..=1 => 1, // unreachable
        2 | 3 => n - 1,
        _ => match n % 3 {
            // get most nimber of 3s
            // and 0..=2 0f 2s
            0 => 3i32.pow(n as u32 / 3),
            1 => 3i32.pow((n - 4) as u32 / 3) * 4,
            2 => 3i32.pow((n - 2) as u32 / 3) * 2,
            _ => unreachable!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(integer_break(2), 1);
        debug_assert_eq!(integer_break(10), 36);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
