mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_taxi_earnings(_n: i32, rides: &mut [[i32; 3]]) -> i64 {
    let n = rides.len();
    rides.sort_unstable_by_key(|v| v[1]);
    let mut dp = vec![0; 1 + n];
    for (idx, ride) in rides.iter().enumerate() {
        let [start, end, tip] = ride[..] else {
            break;
        };
        let curr = i64::from(end - start + tip);
        let i = rides.partition_point(|v| v[1] <= start);
        // Skip current => dp[idx]
        // Take current => pick as late one as possible with binary search
        dp[1 + idx] = dp[idx].max(curr + dp[i]);
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
        assert_eq!(max_taxi_earnings(5, &mut [[2, 5, 4], [1, 5, 1]]), 7);
        assert_eq!(
            max_taxi_earnings(
                20,
                &mut [
                    [1, 6, 1],
                    [3, 10, 2],
                    [10, 12, 3],
                    [11, 12, 2],
                    [12, 15, 2],
                    [13, 18, 1]
                ]
            ),
            20
        );
    }

    #[test]
    fn test() {}
}
