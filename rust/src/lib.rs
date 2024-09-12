mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn wiggle_max_length(nums: &[i32]) -> i32 {
    let (mut incr, mut decr) = (1, 1);
    for w in nums.windows(2) {
        match w[0].cmp(&w[1]) {
            std::cmp::Ordering::Less => incr = decr + 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => decr = incr + 1,
        }
    }
    incr.max(decr)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(wiggle_max_length(&[1, 7, 4, 9, 2, 5]), 6);
        debug_assert_eq!(wiggle_max_length(&[1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
        debug_assert_eq!(wiggle_max_length(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(wiggle_max_length(&[0, 0]), 1);
        debug_assert_eq!(wiggle_max_length(&[0, 0, 0]), 1);
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
