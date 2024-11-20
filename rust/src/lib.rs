mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn take_characters(s: &str, k: i32) -> i32 {
    let counts = s.bytes().fold([0; 3], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    if counts.iter().any(|&v| v < k) {
        return -1;
    }
    let upper = counts.map(|v| v - k);
    let mut window = [0; 3];
    let mut left = 0;
    let mut len = 0;
    for (right, b) in s.bytes().enumerate() {
        window[usize::from(b - b'a')] += 1;
        if window.iter().zip(upper).all(|(&a, b)| a <= b) {
            len = len.max(right - left + 1);
        }
        while window.iter().zip(upper).any(|(&a, b)| a > b) {
            window[usize::from(s.as_bytes()[left] - b'a')] -= 1;
            left += 1;
        }
    }
    (s.len() - len) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(take_characters("aabaaaacaabc", 2), 8);
        debug_assert_eq!(take_characters("a", 1), -1);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
