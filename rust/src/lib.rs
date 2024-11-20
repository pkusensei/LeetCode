mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn camel_match(queries: &[&str], pattern: &str) -> Vec<bool> {
    queries.iter().map(|s| check(s, pattern)).collect()
}

fn check(s: &str, t: &str) -> bool {
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let mut idx = 0;
    for &b in s.iter() {
        if b.is_ascii_uppercase() {
            if t.get(idx) != Some(&b) {
                return false;
            }
            idx += 1;
        } else if t.get(idx).is_some_and(|&v| v == b) {
            idx += 1;
        }
    }
    idx == t.len()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            camel_match(
                &[
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ],
                "FB"
            ),
            [true, false, true, true, false]
        );
        debug_assert_eq!(
            camel_match(
                &[
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ],
                "FoBa"
            ),
            [true, false, true, false, false]
        );
        debug_assert_eq!(
            camel_match(
                &[
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ],
                "FoBaT"
            ),
            [false, true, false, false, false]
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
