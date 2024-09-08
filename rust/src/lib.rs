mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn wiggle_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n == 1 {
        return;
    }
    let mid = (n + 1) / 2;
    let mut temp = nums.to_owned();
    temp.select_nth_unstable(mid); // or sort(_unstable)

    let (mut left, mut right) = (mid - 1, n - 1);
    for (i, v) in nums.iter_mut().enumerate() {
        if i & 1 == 1 {
            *v = temp[right];
            right -= 1;
        } else {
            *v = temp[left];
            left = left.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        {
            let mut nums = vec![1, 5, 1, 1, 6, 4];
            wiggle_sort(&mut nums);
            debug_assert_eq!(nums, [1, 6, 1, 5, 1, 4]);
        }
        {
            let mut nums = vec![1, 3, 2, 2, 3, 1];
            wiggle_sort(&mut nums);
            debug_assert_eq!(nums, [2, 3, 1, 3, 1, 2]);
        }
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
