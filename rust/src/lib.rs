mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_satisfied(customers: &[i32], grumpy: &[i32], minutes: i32) -> i32 {
    let mut curr = 0;
    let m = minutes as usize;
    for (idx, (&c, &g)) in customers.iter().zip(grumpy.iter()).enumerate() {
        if idx < m {
            curr += c;
        } else {
            curr += c * (1 - g);
        }
    }
    let mut res = curr;
    let n = customers.len();
    for idx in 1..=n - m {
        curr -= customers[idx - 1] * grumpy[idx - 1];
        curr += customers[idx + m - 1] * grumpy[idx + m - 1];
        res = res.max(curr)
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_satisfied(&[1, 0, 1, 2, 1, 1, 7, 5], &[0, 1, 0, 1, 0, 1, 0, 1], 3),
            16
        );
        debug_assert_eq!(max_satisfied(&[1], &[0], 1), 1);
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
