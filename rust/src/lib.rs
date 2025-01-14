mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_number_of_frogs(croak_of_frogs: &str) -> i32 {
    let mut count = [0; 4];
    let mut res = 0;
    for b in croak_of_frogs.bytes() {
        match b {
            b'c' => count[0] += 1,
            b'r' => {
                if count[0] > count[1] {
                    count[1] += 1
                } else {
                    return -1;
                }
            }
            b'o' => {
                if count[1] > count[2] {
                    count[2] += 1
                } else {
                    return -1;
                }
            }
            b'a' => {
                if count[2] > count[3] {
                    count[3] += 1
                } else {
                    return -1;
                }
            }
            _ => {
                for v in count.iter_mut() {
                    (*v) -= 1
                }
                if count.iter().any(|&v| v < 0) {
                    return -1;
                }
                res = res.max(count.iter().copied().max().unwrap_or(0));
            }
        }
    }
    if count.into_iter().any(|v| v != 0) {
        -1
    } else {
        1 + res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_number_of_frogs("croakcroak"), 1);
        assert_eq!(min_number_of_frogs("crcoakroak"), 2);
        assert_eq!(min_number_of_frogs("croakcrook"), -1);
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
