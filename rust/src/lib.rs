mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_remove_to_make_valid(s: String) -> String {
    let mut s = s.into_bytes();
    let mut idx = 0;
    let mut open = 0;
    while let Some(&b) = s.get(idx) {
        match b {
            b'(' => open += 1,
            b')' => {
                open -= 1;
                if open < 0 {
                    s.remove(idx);
                    open = 0;
                    continue;
                }
            }
            _ => (),
        }
        idx += 1;
    }
    if !s.is_empty() {
        idx = s.len() - 1;
        let mut close = 0;
        loop {
            match s[idx] {
                b')' => close += 1,
                b'(' => {
                    close -= 1;
                    if close < 0 {
                        s.remove(idx);
                        close = 0;
                    }
                }
                _ => (),
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            min_remove_to_make_valid("lee(t(c)o)de)".into()),
            "lee(t(c)o)de"
        );
        assert_eq!(min_remove_to_make_valid("a)b(c)d".into()), "ab(c)d");
        assert_eq!(min_remove_to_make_valid("))((".into()), "");
    }

    #[test]
    fn test() {
        assert_eq!(min_remove_to_make_valid(")))))".into()), "");
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
