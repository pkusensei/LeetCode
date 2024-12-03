mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn add_spaces(s: String, spaces: &[i32]) -> String {
    // let mut s = s.into_bytes();
    // for i in spaces.iter().rev() {
    //     s.insert(*i as usize, b' ');
    // }
    // String::from_utf8(s).unwrap()
    let mut res = Vec::with_capacity(s.len() + spaces.len());
    let mut i1 = 0;
    for (i2, b) in s.bytes().enumerate() {
        if spaces.get(i1).is_some_and(|&v| v as usize == i2) {
            res.push(b' ');
            i1 += 1;
        }
        res.push(b);
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            add_spaces("LeetcodeHelpsMeLearn".into(), &[8, 13, 15]),
            "Leetcode Helps Me Learn"
        );
        assert_eq!(
            add_spaces("icodeinpython".into(), &[1, 5, 7, 9]),
            "i code in py thon"
        );
        assert_eq!(
            add_spaces("spacing".into(), &[0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g"
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
