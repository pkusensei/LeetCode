mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_min_moves(machines: &[i32]) -> i32 {
    let n = machines.len();
    let sum: i32 = machines.iter().sum();
    if sum % (n as i32) > 0 {
        return -1;
    }
    let average = sum / (n as i32);
    let (mut curr, mut res) = (0, 0);
    // Find max throughput/flow on every spot
    for &num in machines.iter() {
        // Accumulates flow until current spot
        curr += num - average;
        // num - average => givers works one by one
        // while receivers can receive multiple
        // hence no abs() here
        res = res.max(curr.abs()).max(num - average)
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_min_moves(&[1, 0, 5]), 3);
        debug_assert_eq!(find_min_moves(&[0, 3, 0]), 2);
        debug_assert_eq!(find_min_moves(&[0, 2, 0]), -1);
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
