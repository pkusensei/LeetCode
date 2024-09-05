mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; n * 2];
        tree[n..].copy_from_slice(&nums);
        for i in (1..n).rev() {
            tree[i] = tree[i << 1] + tree[i << 1 | 1] // i*2+1
        }
        Self { tree, n }
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut idx = index as usize;
        idx += self.n;
        self.tree[idx] = val;
        while idx > 1 {
            self.tree[idx >> 1] = self.tree[idx] + self.tree[idx ^ 1];
            idx >>= 1;
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (mut left, mut right) = (left as usize + self.n, right as usize + self.n);
        let mut res = 0;
        while left <= right {
            if left & 1 == 1 {
                res += self.tree[left];
                left += 1;
            }
            if right & 1 == 0 {
                res += self.tree[right];
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut arr = NumArray::new(vec![1, 3, 5]);
        debug_assert_eq!(arr.sum_range(0, 2), 9); // return 1 + 3 + 5 = 9
        arr.update(1, 2); // nums = [1, 2, 5]
        debug_assert_eq!(arr.sum_range(0, 2), 8); // return 1 + 2 + 5 = 8
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
