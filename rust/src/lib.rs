mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
    n - no_dups(n)
}

fn no_dups(n: i32) -> i32 {
    let digits = {
        let mut v = vec![];
        let mut n = 1 + n; // to count in n itself
        while n > 0 {
            v.push(n % 10);
            n /= 10;
        }
        v.reverse();
        v
    };
    let len = digits.len();
    // all nums with less "length" than n
    let mut res = (1..len).map(|v| 9 * permutations(9, v as i32 - 1)).sum();

    // all nums with same prefix as n
    let mut seen = std::collections::HashSet::with_capacity(len);
    for (idx, &digit) in digits.iter().enumerate() {
        for d in i32::from(idx == 0)..digit {
            if !seen.contains(&d) {
                res += permutations(9 - idx as i32, (len - idx) as i32 - 1);
            }
        }
        if !seen.insert(digit) {
            break;
        }
    }
    res
}

fn permutations(n: i32, k: i32) -> i32 {
    (n - k + 1..=n).product()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_dup_digits_at_most_n(20), 1);
        debug_assert_eq!(num_dup_digits_at_most_n(100), 10);
        debug_assert_eq!(num_dup_digits_at_most_n(1000), 262);
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
