mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn buddy_strings(s: &str, goal: &str) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    if s == goal {
        goal.bytes()
            .fold([0; 26], |mut acc, b| {
                acc[usize::from(b - b'a')] += 1;
                acc
            })
            .into_iter()
            .any(|v| v > 1)
    } else {
        let mut it = s.bytes().zip(goal.bytes()).filter(|(a, b)| a != b);
        let (Some(a), Some(b)) = (it.next(), it.next()) else {
            return false;
        };
        it.next().is_none() && a.0 == b.1 && a.1 == b.0
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(buddy_strings("ab", "ba"));
        debug_assert!(!buddy_strings("ab", "ab"));
        debug_assert!(buddy_strings("aa", "aa"));
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
