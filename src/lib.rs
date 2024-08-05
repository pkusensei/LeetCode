pub fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let it = heights.iter().copied().enumerate();
        let prevs = smallers(heights, it.clone());
        let mut nexts = smallers(heights, it.clone().rev());
        nexts.reverse();

        it.fold(0, |acc, (idx, num)| {
            let left = prevs[idx].unwrap_or(-1);
            let right = nexts[idx].unwrap_or(heights.len() as i32);
            acc.max((right - left - 1) * num)
        })
}

fn smallers(nums: &[i32], it: impl Iterator<Item = (usize, i32)>) -> Vec<Option<i32>> {
    let mut stack = vec![];
    let mut res = Vec::with_capacity(nums.len());
    for (idx, num) in it {
        while stack.last().is_some_and(|&i| nums[i] >= num) {
            stack.pop();
        }
        res.push(stack.last().copied().map(|i| i as i32));
        stack.push(idx);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_rectangle_area(&[2, 1, 5, 6, 2, 3]), 10);
        debug_assert_eq!(largest_rectangle_area(&[2, 4]), 4);
    }

    #[test]
    fn test() {
        debug_assert_eq!(largest_rectangle_area(&[1, 1]), 2);
        debug_assert_eq!(largest_rectangle_area(&[2, 3]), 4);
    }
}
