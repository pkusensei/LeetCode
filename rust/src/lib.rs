mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let idx = nums[i].unsigned_abs() as usize - 1;
        nums[idx] = -nums[idx].abs();
    }
    nums.iter()
        .enumerate()
        .filter_map(|(i, &n)| if n > 0 { Some(1 + i as i32) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            [5, 6]
        );
        debug_assert_eq!(find_disappeared_numbers(vec![1, 1]), [2]);
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
