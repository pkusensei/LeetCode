mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut res = vec![0; 1 + n];
    let mut pow2 = 1;
    for num in 1..=n {
        if num == pow2 {
            pow2 *= 2;
        }
        res[num] = 1 + res[num - pow2 / 2];
        // res[num] = 1 + res[num & (num - 1)];
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        for i in 1u8..=16 {
            dbg!(i, i.count_ones());
        }
        debug_assert_eq!(count_bits(2), [0, 1, 1]);
        debug_assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            count_bits(16),
            [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1]
        );
    }

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
