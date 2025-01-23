mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_transformable(s: &str, t: &str) -> bool {
    // digits' indices in s
    let mut map = s
        .bytes()
        .enumerate()
        .rev()
        .fold([const { vec![] }; 10], |mut acc, (i, b)| {
            acc[usize::from(b - b'0')].push(i);
            acc
        });
    for b in t.bytes() {
        let digit = usize::from(b - b'0');
        let Some(idx) = map[digit].pop() else {
            return false;
        };
        for d in map.iter().take(digit) {
            // There is d < digit with its pos left to digit(idx)
            if d.last().is_some_and(|&v| v < idx) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(is_transformable("84532", "34852"));
        assert!(is_transformable("34521", "23415"));
        assert!(!is_transformable("12345", "12435"));
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
