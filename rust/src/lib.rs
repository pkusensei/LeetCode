mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct ProductOfNumbers {
    nums: Vec<u64>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        } else {
            match self.nums.last() {
                None | Some(&0) => self.nums.push(num as u64),
                Some(v) => self.nums.push(num as u64 * v),
            };
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let (n, k) = (self.nums.len(), k as usize);
        let Some(i) = n.checked_sub(k) else {
            return 0;
        };
        if i == 0 {
            self.nums[n - 1] as _
        } else {
            (self.nums[n - 1] / self.nums[i - 1]) as _
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut pon = ProductOfNumbers::new();
        pon.add(3); // [3]
        pon.add(0); // [3,0]
        pon.add(2); // [3,0,2]
        pon.add(5); // [3,0,2,5]
        pon.add(4); // [3,0,2,5,4]
        assert_eq!(pon.get_product(2), 20); // return 20. The product of the last 2 numbers is 5 * 4 = 20
        assert_eq!(pon.get_product(3), 40); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
        assert_eq!(pon.get_product(4), 0); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
        pon.add(8); // [3,0,2,5,4,8]
        assert_eq!(pon.get_product(2), 32); // return 32. The product of the last 2 numbers is 4 * 8 = 32
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
