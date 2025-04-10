mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_the_profit(_n: i32, mut offers: Vec<[i32; 3]>) -> i32 {
    let len = offers.len();
    offers.sort_unstable();
    dfs(&offers, 0, &mut vec![-1; len])
}

fn dfs(offers: &[[i32; 3]], idx: usize, memo: &mut [i32]) -> i32 {
    let len = offers.len();
    if idx >= len {
        return 0;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let skip = dfs(offers, 1 + idx, memo);
    let mut left = 1 + idx;
    let mut right = len - 1;
    let mut next = len;
    while left <= right {
        let mid = left + (right - left) / 2;
        if offers[mid][0] > offers[idx][1] {
            right = mid - 1;
            next = mid;
        } else {
            left = mid + 1;
        }
    }
    let take = offers[idx][2] + dfs(offers, next, memo);
    memo[idx] = skip.max(take);
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
    fn basics() {
        assert_eq!(
            maximize_the_profit(5, vec![[0, 0, 1], [0, 2, 2], [1, 3, 2]]),
            3
        );
        assert_eq!(
            maximize_the_profit(5, vec![[0, 0, 1], [0, 2, 10], [1, 3, 2]]),
            10
        );
    }

    #[test]
    fn test() {}
}
