mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_hamming_distance(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    // ham_dist is the number of ones after a^b
    // Consider a single bit position
    // Each 1 with a 0 would produce a 1 after xor
    // Thus on this bit count(1)*count(0) is its contribution
    // Not do that for all 32 bits
    for bit in 0..32 {
        let mut count = 0;
        for &num in nums.iter() {
            if num & (1 << bit) > 0 {
                count += 1
            }
        }
        res += count * (n as i32 - count);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(total_hamming_distance(&[4, 14, 2]), 6);
        debug_assert_eq!(total_hamming_distance(&[4, 14, 4]), 4);
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
