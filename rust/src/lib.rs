mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum_of_heights(max_heights: &[i32]) -> i64 {
    use itertools::izip;
    let n = max_heights.len();
    let mut stack = vec![];
    let mut next_smaller = vec![None; n];
    for (idx, &num) in max_heights.iter().enumerate() {
        while stack.last().is_some_and(|&i| max_heights[i] >= num) {
            let top = stack.pop().unwrap();
            next_smaller[top] = Some(idx);
        }
        stack.push(idx);
    }
    stack.clear();
    let mut prev_smaller = vec![None; n];
    for (idx, &num) in max_heights.iter().enumerate().rev() {
        while stack.last().is_some_and(|&i| max_heights[i] >= num) {
            let top = stack.pop().unwrap();
            prev_smaller[top] = Some(idx);
        }
        stack.push(idx);
    }
    let mut left_dp = vec![0; n];
    for (idx, &num) in max_heights.iter().enumerate() {
        let num = i64::from(num);
        if let Some(prev) = prev_smaller[idx] {
            left_dp[idx] = left_dp[prev] + (idx - prev) as i64 * num;
        } else {
            left_dp[idx] = (1 + idx) as i64 * num;
        }
    }
    let mut right_dp = vec![0; n];
    for (idx, &num) in max_heights.iter().enumerate().rev() {
        let num = i64::from(num);
        if let Some(next) = next_smaller[idx] {
            right_dp[idx] = right_dp[next] + (next - idx) as i64 * num;
        } else {
            right_dp[idx] = (n - idx) as i64 * num;
        }
    }
    izip!(left_dp, right_dp, max_heights.iter())
        .map(|(a, b, &c)| a + b - i64::from(c))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(maximum_sum_of_heights(&[5, 3, 4, 1, 1]), 13);
        assert_eq!(maximum_sum_of_heights(&[6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(maximum_sum_of_heights(&[3, 2, 5, 5, 2, 3]), 18);
    }

    #[test]
    fn test() {}
}
