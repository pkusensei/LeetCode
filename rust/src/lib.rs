mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_mountain_removals(nums: &[i32]) -> i32 {
    let n = nums.len();
    let (mut inc, mut dec) = (vec![1; n], vec![1; n]);
    for i1 in 1..n {
        for i2 in 0..i1 {
            if nums[i2] < nums[i1] {
                inc[i1] = inc[i1].max(1 + inc[i2]);
            }
        }
    }
    for i1 in (0..n).rev() {
        for i2 in 1 + i1..n {
            if nums[i1] > nums[i2] {
                dec[i1] = dec[i1].max(1 + dec[i2]);
            }
        }
    }
    let mount = inc
        .into_iter()
        .zip(dec)
        .filter_map(|(a, b)| {
            if a > 1 && b > 1 {
                Some(a + b - 1)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(1);
    nums.len() as i32 - mount
}

fn with_binary_search(nums: &mut [i32]) -> i32 {
    let inc = lis(nums);
    nums.reverse();
    let mut dec = lis(nums);
    dec.reverse();

    let mount = inc
        .into_iter()
        .zip(dec)
        .filter_map(|(a, b)| {
            if a > 1 && b > 1 {
                Some(a + b - 1)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(1);
    nums.len() as i32 - mount
}

fn lis(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![1; n]; // track length of LIS by each index
    let mut lis = vec![nums[0]]; // track LIS
    for (idx, &num) in nums.iter().enumerate().skip(1) {
        // binary search to find num in lis
        let i = lis.partition_point(|&v| v < num);
        if i == lis.len() {
            // num is bigger than all LIS
            lis.push(num);
        } else {
            // update num into LIS
            lis[i] = num;
        }
        res[idx] = lis.len() as i32;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_binary_search(&mut [1, 3, 1]), 0);
        debug_assert_eq!(with_binary_search(&mut [2, 1, 1, 5, 6, 2, 3, 1]), 3);
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
