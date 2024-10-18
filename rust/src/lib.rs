mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reach_number(target: i32) -> i32 {
    let target = target.abs();
    let mut curr = 0;
    let mut num = 0;
    while curr < target {
        num += 1;
        curr += num;
    }
    let delta = curr - target;
    if delta & 1 == 0 {
        // delta == 0 => num
        // delta is even => find number in [1..=num] and flip its sign
        num
    } else if num & 1 == 0 {
        // num is even, but delta is odd
        // (num+1) is odd
        // Find an odd number in [1..=1+num] to offset delta
        num + 1
    } else {
        // delta is odd, num is odd
        // curr and target are both odd
        // Flipping any odd number instead makes curr even
        // Has to flip _two_ numbers
        num + 2
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reach_number(2), 3);
        debug_assert_eq!(reach_number(3), 2);
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
