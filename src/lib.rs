use std::collections::HashSet;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = candidates;
    nums.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
    solve(&nums, target).into_iter().collect()
}

fn solve(nums: &[i32], target: i32) -> HashSet<Vec<i32>> {
    match nums {
        [] => HashSet::new(),
        [head, tail @ ..] => match target.cmp(head) {
            std::cmp::Ordering::Less => solve(tail, target),
            std::cmp::Ordering::Equal => {
                let mut res = solve(tail, target);
                res.insert(vec![*head]);
                res
            }
            std::cmp::Ordering::Greater => {
                let mut res = solve(tail, target);
                res.extend(solve(tail, target - head).into_iter().map(|mut v| {
                    v.push(*head);
                    v
                }));
                res.extend(solve(nums, target - head).into_iter().map(|mut v| {
                    v.push(*head);
                    v
                }));
                res
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            [vec![2, 2, 3], vec![7]]
        );
        debug_assert_eq!(
            combination_sum(vec![2, 3, 5], 8),
            [vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        debug_assert_eq!(combination_sum(vec![2], 1), Vec::<Vec<i32>>::new())
    }

    #[test]
    fn test() {}
}
