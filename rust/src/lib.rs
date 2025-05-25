mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(n: i32, k: i32, stay_score: &[&[i32]], travel_score: &[&[i32]]) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    let mut dp = vec![0; n];
    for i in 0..k {
        let mut curr = vec![0; n];
        for dest in 0..n {
            curr[dest] = curr[dest].max(dp[dest] + stay_score[i][dest]);
            for city in 0..n {
                curr[dest] = curr[dest].max(dp[city] + travel_score[city][dest]);
            }
        }
        dp = curr;
    }
    dp.into_iter().max().unwrap()
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
        assert_eq!(max_score(2, 1, &[&[2, 3]], &[&[0, 2], &[1, 0]]), 3);
        assert_eq!(
            max_score(
                3,
                2,
                &[&[3, 4, 2], &[2, 1, 2]],
                &[&[0, 2, 1], &[2, 0, 4], &[3, 2, 0]]
            ),
            8
        );
    }

    #[test]
    fn test() {}
}
