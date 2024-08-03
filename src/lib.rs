pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let i = intervals
        .iter()
        .position(|v| v[0] > new_interval[0])
        .unwrap_or(intervals.len());
    intervals.insert(i, new_interval);
    merge(intervals)
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }
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
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            [[1, 5], [6, 9]]
        );
        debug_assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            [[1, 2], [3, 10], [12, 16]]
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(merge(vec![vec![1, 4], vec![2, 3]]), [[1, 4]]);
    }
}
