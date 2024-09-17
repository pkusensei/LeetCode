mod helper;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_xor(nums: &[i32]) -> i32 {
    let (mut res, mut mask) = (0, 0);
    // working bit by bit, start from most significant
    for bit in (0..32).rev() {
        mask |= 1 << bit;
        // keep a record of numbers that's 1 on current bit
        let set: HashSet<_> = nums.iter().map(|n| n & mask).collect();
        // attempt to set current bit to 1
        let trial = res | (1 << bit);
        for left_bits in set.iter() {
            // a^b=c => a^c=b
            // to find n1 and n2 so that n1^n2=max
            // n1 is in set, try find max^n2
            let seek = trial ^ left_bits;
            if set.contains(&seek) {
                // current bit could be set
                // record it in res
                res = trial;
                // proceed to next bit
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_maximum_xor(&[3, 10, 5, 25, 2, 8]), 28);
        debug_assert_eq!(
            find_maximum_xor(&[14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
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
}
