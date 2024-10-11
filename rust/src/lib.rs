mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_of_three_subarrays(nums: &[i32], k: i32) -> Vec<i32> {
        let k = k as usize;
        // let n = nums.len();
        // let mut sums = Vec::with_capacity(n - k);
        // let mut temp: i32 = nums[..k].iter().sum();
        // sums.push(temp);
        // for i in 1..=n - k {
        //     temp -= nums[i - 1];
        //     temp += nums[i + k - 1];
        //     sums.push(temp);
        // }
        let sums: Vec<i32> = nums.windows(k).map(|w| w.iter().sum()).collect();
        let mut max_i = 0;
        let mut left_max = Vec::with_capacity(sums.len());
        for (i, &num) in sums.iter().enumerate() {
            if num > sums[max_i] {
                max_i = i;
            }
            left_max.push(max_i);
        }

        max_i = sums.len() - 1;
        let mut right_max = Vec::with_capacity(sums.len());
        for (i, &num) in sums.iter().enumerate().rev() {
            if num >= sums[max_i] {
                max_i = i;
            }
            right_max.push(max_i);
        }
        right_max.reverse();

        let mut res = [0; 3];
        let mut curr = 0;
        for idx in k..sums.len() - k {
            let left = sums[left_max[idx - k]];
            let right = sums[right_max[idx + k]];
            let temp = sums[idx] + left + right;
            if temp > curr {
                res = [left_max[idx - k], idx, right_max[idx + k]];
                curr = temp
            }
        }
        res.map(|i| i as i32).to_vec()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_sum_of_three_subarrays(&[1, 2, 1, 2, 6, 7, 5, 1], 2),
            [0, 3, 5]
        );
        debug_assert_eq!(
            max_sum_of_three_subarrays(&[1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            [0, 2, 4]
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            max_sum_of_three_subarrays(&[4, 5, 10, 6, 11, 17, 4, 11, 1, 3], 1),
            [4, 5, 7]
        );
    }

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
