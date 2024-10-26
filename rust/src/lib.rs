mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_friend_requests(ages: &mut [i32]) -> i32 {
    ages.sort_unstable();
    let mut res = 0;
    for &num in ages.iter() {
        let min = num / 2 + 7;
        if min < num {
            let left = ages.partition_point(|&n| n <= min);
            let right = ages.partition_point(|&n| n <= num);
            res += right.saturating_sub(1 + left);
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(num_friend_requests(&mut [16, 16]), 2);
        debug_assert_eq!(num_friend_requests(&mut [16, 17, 18]), 2);
        debug_assert_eq!(num_friend_requests(&mut [20, 30, 100, 110, 120]), 3);
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
