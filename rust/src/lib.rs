mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_insertions(s: &str) -> i32 {
    let mut res = 0;
    let [mut open, mut close] = [0, 0];
    for b in s.bytes() {
        match b {
            b'(' => {
                if open > 0 && close == 1 {
                    res += 1;
                    close = 0; // add one ) to balance ())
                } else {
                    open += 1
                }
            }
            _ => {
                close += 1;
                if open == 0 {
                    res += 1;
                    open += 1;
                }
                if close == 2 {
                    open -= 1;
                    close = 0;
                }
            }
        }
    }
    res + 2 * open - close
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_insertions("(()))"), 1);
        assert_eq!(min_insertions("())"), 0);
        assert_eq!(min_insertions("))())("), 3);
    }

    #[test]
    fn test() {
        assert_eq!(min_insertions(")))))))"), 5);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
