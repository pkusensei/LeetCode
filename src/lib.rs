pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = candidates;
    nums.sort_unstable();
    let mut res = vec![];
    backtrace(&nums, target, &mut res, &mut vec![], 0);
    res
}

fn backtrace(
    nums: &[i32],
    target: i32,
    result: &mut Vec<Vec<i32>>,
    curr: &mut Vec<i32>,
    start: usize,
) {
    if target == 0 {
        result.push(curr.clone());
    } else {
        for i in start..nums.len() {
            if target < nums[i] {
                break;
            }
            if i == start || nums[i] != nums[i - 1] {
                curr.push(nums[i]);
                backtrace(nums, target - nums[i], result, curr, i + 1);
                curr.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            [vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        debug_assert_eq!(
            combination_sum2(vec![2, 5, 2, 1, 2], 5),
            [vec![1, 2, 2], vec![5]]
        );
        debug_assert_eq!(combination_sum2(vec![2], 1), Vec::<Vec<i32>>::new())
    }

    #[test]
    fn test() {}
}
