mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_points(points: &[&[i32]]) -> i64 {
    let (row, col) = (points.len(), points.first().map(|r| r.len()).unwrap_or(0));
    if row * col == 0 {
        return 0;
    }
    if row == 1 {
        return points[0].iter().max().copied().unwrap_or(0) as _;
    }
    if col == 1 {
        return (0..row).map(|n| i64::from(points[n][0])).sum();
    }
    let mut prev: Vec<i64> = points[0].iter().copied().map(i64::from).collect();
    for r in points.iter().skip(1) {
        let mut left_max = vec![prev[0]];
        for x in 1..col {
            left_max.push((left_max[x - 1] - 1).max(prev[x]));
        }
        let mut right_max = vec![0; col - 1];
        right_max.push(prev[col - 1]);
        for x in (0..=col - 2).rev() {
            right_max[x] = (right_max[x + 1] - 1).max(prev[x]);
        }
        let curr = r
            .iter()
            .enumerate()
            .map(|(x, &num)| num as i64 + left_max[x].max(right_max[x]))
            .collect();
        prev = curr;
    }
    prev.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_points(&[&[1, 2, 3], &[1, 5, 1], &[3, 1, 1]]), 9);
        debug_assert_eq!(max_points(&[&[1, 5], &[2, 3], &[4, 2]]), 11);
    }

    #[test]
    fn test() {
        debug_assert_eq!(max_points(&[&[1], &[2], &[3]]), 6)
    }
}
