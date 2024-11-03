mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_special_equiv_groups(words: &[&str]) -> i32 {
    words
        .iter()
        .map(|s| count(s))
        .collect::<std::collections::HashSet<_>>()
        .len() as i32
}

fn count(s: &str) -> [[i32; 26]; 2] {
    let [mut even, mut odd] = [0, 1].map(|_| [0; 26]);
    for (idx, b) in s.bytes().enumerate() {
        let i = usize::from(b - b'a');
        if idx & 1 == 1 {
            odd[i] += 1;
        } else {
            even[i] += 1;
        }
    }
    [even, odd]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_special_equiv_groups(&["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]),
            3
        );
        debug_assert_eq!(
            num_special_equiv_groups(&["abc", "acb", "bac", "bca", "cab", "cba"]),
            3
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
