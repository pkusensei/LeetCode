mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_boxes(boxes: &[i32]) -> i32 {
    let n = boxes.len();
    let mut dp = vec![vec![vec![0; n]; n]; n];
    solve(boxes, &mut dp, 0, n - 1, 0)
}

fn solve(nums: &[i32], dp: &mut [Vec<Vec<i32>>], left: usize, right: usize, count: usize) -> i32 {
    if left > right {
        return 0;
    }
    if dp[left][right][count] > 0 {
        return dp[left][right][count];
    }
    let (mut c_left, mut c_count) = (left, count);
    while c_left < right && nums[c_left] == nums[c_left + 1] {
        c_left += 1;
        c_count += 1;
    }
    // [1,1,1,2,2,1..]
    // remove [1,1,1] and calculate [2,2,1...]
    let mut res = ((c_count + 1) * (c_count + 1)) as i32 + solve(nums, dp, c_left + 1, right, 0);

    for i in c_left + 1..=right {
        if nums[i] == nums[c_left] {
            // find all chance to merge
            // [1,1,1,2,2,1,1..] can be seen as
            // [2,2] + [1,1,1,1,1..]
            //          ^^^^^ to carry over this information
            //                add count to the next recursion
            res = res
                .max(solve(nums, dp, c_left + 1, i - 1, 0) + solve(nums, dp, i, right, c_count + 1))
        }
    }
    dp[left][right][count] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(remove_boxes(&[1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
        debug_assert_eq!(remove_boxes(&[1, 1, 1]), 9);
        debug_assert_eq!(remove_boxes(&[1]), 1);
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
