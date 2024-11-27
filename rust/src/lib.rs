mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_ocurrences(text: &str, first: &str, second: &str) -> Vec<String> {
        text.split_ascii_whitespace()
            .collect::<Vec<_>>()
            .windows(3)
            .filter_map(|w| {
                if w[0] == first && w[1] == second {
                    Some(w[2].to_string())
                } else {
                    None
                }
            })
            .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_ocurrences("alice is a good girl she is a good student", "a", "good"),
            ["girl", "student"]
        );
        debug_assert_eq!(
            find_ocurrences("we will we will rock you", "we", "will"),
            ["we", "rock"]
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
