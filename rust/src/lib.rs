mod helper;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition_k_subsets(nums: &mut [i32], k: i32) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % k > 0 {
        return false;
    }
    nums.sort_unstable_by_key(|&n| Reverse(n));
    // let target = sum / k;
    // let k = k as usize;
    // let mut subs = vec![0; k];
    // backtrack(nums, &mut subs, k, target)

    let n = nums.len();
    let mut dp = vec![-1; 1 << n];
    dp[0] = 0;
    // each num in nums has two states: 1<<n
    for mask in 0..1 << n {
        if dp[mask] == -1 {
            // not reachable
            continue;
        }
        let rem = sum - (dp[mask] % sum); // ???
        for (idx, &num) in nums.iter().enumerate() {
            let temp = mask | 1 << idx;
            if temp != mask && dp[temp] < 0 && nums[idx] <= rem {
                dp[temp] = dp[mask] + num;
            }
        }
    }
    *dp.last().unwrap() == sum
}

fn backtrack(nums: &[i32], subs: &mut [i32], k: usize, target: i32) -> bool {
    match nums {
        [] => true,
        [head, tail @ ..] => {
            for i in 0..k {
                if subs[i] + head <= target {
                    subs[i] += head;
                    if backtrack(tail, subs, k, target) {
                        return true;
                    }
                    subs[i] -= head;
                    if subs[i] == 0 {
                        // After backtracking this subset is empty
                        // i.e there's no further combinations to yield a true
                        // All empty subsets are the same/symmetric
                        break;
                    }
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_partition_k_subsets(&mut [4, 3, 2, 3, 5, 2, 1], 4));
        debug_assert!(!can_partition_k_subsets(&mut [1, 2, 3, 4], 3));
    }

    #[test]
    fn test() {
        assert!(can_partition_k_subsets(
            &mut [
                3522, 181, 521, 515, 304, 123, 2512, 312, 922, 407, 146, 1932, 4037, 2646, 3871,
                269
            ],
            5
        ));
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
