mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reorder_log_files<'a>(logs: &[&'a str]) -> Vec<&'a str> {
    let (mut letters, mut digits) = (vec![], vec![]);
    for s in logs.iter() {
        let Some((_a, b)) = s.split_once(' ') else {
            continue;
        };
        if b.bytes().any(|byte| byte.is_ascii_lowercase()) {
            letters.push(*s);
        } else {
            digits.push(*s);
        }
    }
    letters.sort_unstable_by(|x, y| {
        let (a1, b1) = x.split_once(' ').unwrap();
        let (a2, b2) = y.split_once(' ').unwrap();
        b1.cmp(b2).then(a1.cmp(a2))
    });
    letters.extend(digits);
    letters
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            reorder_log_files(&[
                "dig1 8 1 5 1",
                "let1 art can",
                "dig2 3 6",
                "let2 own kit dig",
                "let3 art zero"
            ]),
            [
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6"
            ]
        );
        debug_assert_eq!(
            reorder_log_files(&[
                "a1 9 2 3 1",
                "g1 act car",
                "zo4 4 7",
                "ab1 off key dog",
                "a8 act zoo"
            ]),
            [
                "g1 act car",
                "a8 act zoo",
                "ab1 off key dog",
                "a1 9 2 3 1",
                "zo4 4 7"
            ]
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
