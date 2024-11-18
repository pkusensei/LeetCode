mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn clumsy(mut n: i32) -> i32 {
    let mut op = 0; // [* / + -]
    let mut num1 = n;
    let mut num2 = 0;
    n -= 1;
    while n > 0 {
        match op {
            0 => num1 *= n,
            1 => num1 /= n,
            2 => {
                num2 += num1 + n;
                num1 = 0;
            }
            3 => num1 = -n,
            _ => unreachable!(),
        }
        n -= 1;
        op = (op + 1) % 4;
    }
    num1 + num2
}

const fn pattern(n: i32) -> i32 {
    match n {
        1 | 2 => n,
        3 => 6,
        4 => 7,
        _ => match n % 4 {
            1 | 2 => 2 + n,
            3 => n - 1,
            _ => 1 + n,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(clumsy(4), 7);
        debug_assert_eq!(clumsy(10), 12);
        debug_assert_eq!(pattern(4), 7);
        debug_assert_eq!(pattern(10), 12);
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
