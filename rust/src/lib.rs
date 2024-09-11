mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn super_pow(mut a: i32, b: &[i32]) -> i32 {
    if a == 1 {
        return 1;
    }
    let mut res = 1;
    for &n in b.iter().rev() {
        res = (res * pow_mod(a, n)) % MOD;
        a = pow_mod(a, 10);
    }
    res
}

// a**(b1*10+b2) = a**(b1*10) * a**b2 = a**10 * a**b1 * a**b2
// ab % m = (a%m)(b%m)%m
const MOD: i32 = 1337;

fn pow_mod(mut a: i32, b: i32) -> i32 {
    a %= MOD;
    (0..b).fold(1, |acc, _| (acc * a) % MOD)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(super_pow(2, &[3]), 8);
        debug_assert_eq!(super_pow(2, &[1, 0]), 1024);
        debug_assert_eq!(super_pow(1, &[4, 3, 3, 8, 5, 2]), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
