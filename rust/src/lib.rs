mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn elevator_requests(_n: i32, start: i32, requests: Vec<Vec<i32>>) -> i64 {
    let n = requests.len();
    let full = 1 << n;
    let mut dp = vec![vec![i64::MAX >> 2; n]; full];
    dp[0].fill(0);
    for mask in 1..full {
        for i in 0..n {
            if mask & (1 << i) == 0 {
                continue;
            }
            let [arr, floor] = requests[i][..] else {
                unreachable!()
            };
            let prev_mask = mask ^ (1 << i);
            if prev_mask == 0 {
                dp[mask][i] = i64::from(arr.max((start - floor).abs()));
            } else {
                for prev in 0..n {
                    if prev_mask & (1 << prev) == 0 {
                        continue;
                    }
                    let prev_val = dp[prev_mask][prev];
                    let val =
                        (prev_val + i64::from(requests[prev][1] - floor).abs()).max(i64::from(arr));
                    dp[mask][i] = dp[mask][i].min(val);
                }
            }
        }
    }
    *dp[full - 1].iter().min().unwrap()
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
