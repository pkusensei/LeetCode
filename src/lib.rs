pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    k_sum(&nums, target as _, 0, 4)
}

fn k_sum(nums: &[i32], target: i64, start: usize, k: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if start == nums.len() {
        return res;
    }

    let average = target / k as i64;
    if nums[start] as i64 > average || nums.last().is_some_and(|&n| (n as i64) < average) {
        return res;
    }

    if k == 2 {
        return two_sum(nums, target, start);
    }

    for i in start..nums.len() {
        if i == start || nums.get(i - 1).is_some_and(|&n| n != nums[i]) {
            for subset in k_sum(nums, target - nums[i] as i64, i + 1, k - 1) {
                let mut v = vec![nums[i]];
                v.extend(subset);
                res.push(v)
            }
        }
    }

    res
}

fn two_sum(nums: &[i32], target: i64, start: usize) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let (mut left, mut right) = (start, nums.len() - 1);
    while left < right {
        let curr = nums[left] + nums[right];
        if (curr as i64) < target || (left > start && nums[left] == nums[left - 1]) {
            // curr is smaller || skip repeating numbers
            left += 1
        } else if curr as i64 > target || (right < nums.len() - 1 && nums[right] == nums[right + 1])
        {
            right -= 1;
        } else {
            res.push(vec![nums[left], nums[right]]);
            left += 1;
            right -= 1
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        {
            let mut res: Vec<_> = four_sum(vec![1, 0, -1, 0, -2, 2], 0)
                .into_iter()
                .map(|mut v| {
                    v.sort_unstable();
                    v
                })
                .collect();
            res.sort_unstable();
            debug_assert_eq!(
                res,
                vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
            )
        };
        {
            let mut res: Vec<_> = four_sum(vec![2, 2, 2, 2, 2], 8)
                .into_iter()
                .map(|mut v| {
                    v.sort_unstable();
                    v
                })
                .collect();
            res.sort_unstable();
            debug_assert_eq!(res, vec![vec![2, 2, 2, 2]])
        };
    }

    #[test]
    fn test() {
        {
            let mut res: Vec<_> = four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0)
                .into_iter()
                .map(|mut v| {
                    v.sort_unstable();
                    v
                })
                .collect();
            res.sort_unstable();
            debug_assert_eq!(res, vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]])
        };
        {
            let mut res: Vec<_> = four_sum(
                vec![
                    -1000000000,
                    -1000000000,
                    1000000000,
                    -1000000000,
                    -1000000000,
                ],
                294967296,
            )
            .into_iter()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect();
            res.sort_unstable();
            debug_assert_eq!(res, Vec::<Vec<_>>::new())
        };
    }
}
