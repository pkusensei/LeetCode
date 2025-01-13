mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
    let mut res = 0;
    while k > 0 {
        let temp = fib(k);
        k -= temp;
        res += 1;
    }
    res
}

const fn fib(n: i32) -> i32 {
    let [mut a, mut b] = [1, 1];
    loop {
        let next = a + b;
        if next > n {
            break;
        }
        a = b;
        b = next;
    }
    b
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(find_min_fibonacci_numbers(7), 2);
        assert_eq!(find_min_fibonacci_numbers(10), 2);
        assert_eq!(find_min_fibonacci_numbers(19), 3);
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
