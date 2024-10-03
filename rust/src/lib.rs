mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_place_flowers(mut nums: Vec<i32>, n: i32) -> bool {
    nums.reserve(4);
    nums.insert(0, 0);
    nums.insert(0, 1); // add [1,0] to the front
    nums.push(0);
    nums.push(1); // add [0,1] to the end
    let is: Vec<_> = nums
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| if n == 1 { Some(i) } else { None })
        .collect();
    let count: usize = is
        .windows(2)
        .map(|w| w[1] - w[0] - 1)
        .filter_map(|n| if n > 2 { Some((n - 1) / 2) } else { None })
        .sum();
    count >= n as usize
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        debug_assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
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
