mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_swap(s1: &str, s2: &str) -> i32 {
    let mut pairs: Vec<_> = s1.bytes().zip(s2.bytes()).filter(|(a, b)| a != b).collect();
    let mut res = 0;
    if pairs.is_empty() {
        return res;
    }
    pairs.sort_unstable();
    while pairs.len() >= 2 && pairs[0] == pairs[1] {
        res += 1;
        pairs.drain(..2);
    }
    if pairs.len() >= 2 {
        pairs.reverse();
        while pairs.len() >= 2 && pairs[0] == pairs[1] {
            res += 1;
            pairs.drain(..2);
        }
    }
    match pairs.len() {
        2 => res + 2,
        0 => res,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(minimum_swap("xx", "yy"), 1);
        assert_eq!(minimum_swap("xy", "yx"), 2);
        assert_eq!(minimum_swap("xx", "xy"), -1);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_swap("yxyxxxyyxxyxxxx", "yyyxyyyxyxxxyxy"), 4);
    }

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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
