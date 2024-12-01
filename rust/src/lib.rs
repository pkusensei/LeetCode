mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn tribonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let n = n as usize;
            let mut f = vec![0; 1 + n];
            f[1] = 1;
            f[2] = 1;
            for i in 3..=n {
                f[i] = f[i - 3] + f[i - 2] + f[i - 1];
            }
            f[n]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(tribonacci(4), 4);
        debug_assert_eq!(tribonacci(25), 1389537);
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
