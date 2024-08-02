pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }
    let mut intervals = intervals;
    intervals.sort_unstable();
    let mut res = vec![];
    let mut curr = intervals[0].to_vec();
    for it in intervals.into_iter().skip(1) {
        if it[0] <= curr[1] {
            curr[1] = curr[1].max(it[1])
        } else {
            res.push(curr);
            curr = it
        }
    }
    res.push(curr);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            [[1, 6], [8, 10], [15, 18]]
        );
        debug_assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), [[1, 5]]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(merge(vec![vec![1, 4], vec![2, 3]]), [[1, 4]]);
    }
}
