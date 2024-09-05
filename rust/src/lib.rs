mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct NumArray {
    nums: Vec<i32>,
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len()];
        prefix[0] = nums[0];
        for (idx, &num) in nums.iter().enumerate().skip(1) {
            prefix[idx] = num + prefix[idx - 1];
        }
        Self { nums, prefix }
    }

    fn update(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        if val == self.nums[idx] {
            return;
        }
        for v in self.prefix.iter_mut().skip(idx) {
            *v += val - self.nums[idx];
        }
        self.nums[idx] = val;
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
        let mut arr = NumArray::new(vec![1, 3, 5]);
        arr.sum_range(0, 2); // return 1 + 3 + 5 = 9
        arr.update(1, 2); // nums = [1, 2, 5]
        arr.sum_range(0, 2); // return 1 + 2 + 5 = 8
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
