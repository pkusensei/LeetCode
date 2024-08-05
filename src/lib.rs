pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let size = 1 << nums.len();
    nums.sort_unstable();
    let mut res = std::collections::HashSet::new();
    for mask in size..size * 2 {
        let mut bismask = format!("{:b}", mask);
        bismask.remove(0);
        let curr = (0..nums.len())
            .filter_map(|i| {
                if bismask.as_bytes()[i] == b'1' {
                    Some(nums[i])
                } else {
                    None
                }
            })
            .collect();
        res.insert(curr);
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            subsets_with_dup(vec![1, 2, 2]),
            [
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );
        debug_assert_eq!(subsets_with_dup(vec![0]), [vec![], vec![0]]);
    }

    #[test]
    fn test() {}
}
