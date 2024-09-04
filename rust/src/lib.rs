mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: &[i32]) -> Self {
        let mut prefix = vec![0; nums.len()];
        prefix[0] = nums[0];
        for (idx, &num) in nums.iter().enumerate().skip(1) {
            prefix[idx] = num + prefix[idx - 1];
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        if left < 1 {
            self.prefix[right]
        } else {
            self.prefix[right] - self.prefix[left - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let arr = NumArray::new(&[-2, 0, 3, -5, 2, -1]);
        debug_assert_eq!(arr.sum_range(0, 2), 1); // return (-2) + 0 + 3 = 1
        debug_assert_eq!(arr.sum_range(2, 5), -1); // return 3 + (-5) + 2 + (-1) = -1
        debug_assert_eq!(arr.sum_range(0, 5), -3); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
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
