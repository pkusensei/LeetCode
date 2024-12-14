mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_subarrays(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut odd_counts = vec![0; 1 + n];
    odd_counts[0] = 1;
    let mut curr = 0;
    let mut res = 0;
    for &num in nums.iter() {
        curr += num & 1;
        if curr >= k {
            res += odd_counts[(curr - k) as usize];
        }
        odd_counts[curr as usize] += 1;
    }
    res
}

fn with_simplified_queue(nums: &[i32], k: i32) -> i32 {
    let mut res = 0;
    let (mut qsize, mut left, mut init_gap) = (0, 0, 0);
    for &num in nums.iter() {
        qsize += num & 1;
        if qsize == k {
            init_gap = 0;
            while qsize == k {
                qsize -= nums[left] & 1;
                init_gap += 1;
                left += 1;
            }
        }
        res += init_gap;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(with_simplified_queue(&[1, 1, 2, 1, 1], 3), 2);
        assert_eq!(with_simplified_queue(&[2, 4, 6], 1), 0);
        assert_eq!(
            with_simplified_queue(&[2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
