mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut dp = vec![0; n];
    for idx in (0..n).rev() {
        let [pts, bra] = questions[idx][..] else {
            unreachable!()
        };
        let next = bra as usize + idx + 1;
        let take = i64::from(pts) + if next < n { dp[next] } else { 0 };
        let skip = if 1 + idx < n { dp[1 + idx] } else { 0 };
        dp[idx] = take.max(skip);
    }
    dp[0]
    // dfs(&questions, 0, &mut vec![-1; n])
}

fn dfs(questions: &[Vec<i32>], idx: usize, memo: &mut [i64]) -> i64 {
    if idx >= questions.len() {
        return 0;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let [pts, bra] = questions[idx][..] else {
        unreachable!()
    };
    let take = i64::from(pts) + dfs(questions, 1 + idx + bra as usize, memo);
    let skip = dfs(questions, 1 + idx, memo);
    memo[idx] = take.max(skip);
    memo[idx]
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
    fn basics() {}

    #[test]
    fn test() {}
}
