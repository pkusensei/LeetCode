pub fn maximal_rectangle(matrix: &[&[char]]) -> i32 {
    let row = matrix.len();
    let col = matrix.first().map(|r| r.len()).unwrap_or_default();
    if row * col == 0 {
        return 0;
    }

    let (_, area) = matrix.iter().fold((vec![0; col], 0), |(nums, area), line| {
        let nums: Vec<_> = nums
            .into_iter()
            .zip(line.iter())
            .map(|(n, ch)| if *ch == '0' { 0 } else { n + 1 })
            .collect();
        let area = area.max(largest_rectangle_area(&nums));
        (nums, area)
    });
    area
}

fn largest_rectangle_area(nums: &[i32]) -> i32 {
    let it = nums.iter().copied().enumerate();
    let prevs = smallers(nums, it.clone());
    let mut nexts = smallers(nums, it.clone().rev());
    nexts.reverse();

    it.fold(0, |acc, (idx, num)| {
        let left = prevs[idx].unwrap_or(-1);
        let right = nexts[idx].unwrap_or(nums.len() as i32);
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
        debug_assert_eq!(
            maximal_rectangle(&[
                &['1', '0', '1', '0', '0'],
                &['1', '0', '1', '1', '1'],
                &['1', '1', '1', '1', '1'],
                &['1', '0', '0', '1', '0']
            ]),
            6
        );
        debug_assert_eq!(maximal_rectangle(&[&['0']]), 0);
        debug_assert_eq!(maximal_rectangle(&[&['1']]), 1);
    }

    #[test]
    fn test() {}
}
