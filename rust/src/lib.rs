mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_alternating_sum(nums: &[i32]) -> i64 {
    let n = nums.len();
    dfs(nums, 1, &mut vec![vec![-1; 1 + n]; 2])
}

// pick unpick
fn dfs(nums: &[i32], even: usize, memo: &mut [Vec<i64>]) -> i64 {
    match nums {
        [] => 0,
        [head, tail @ ..] => {
            let n = nums.len();
            if memo[even][n] > -1 {
                return memo[even][n];
            }
            let curr = if even == 1 { *head } else { -head };
            let skip = dfs(tail, even, memo);
            let take = i64::from(curr) + dfs(tail, 1 - even, memo);
            let res = skip.max(take);
            memo[even][n] = res;
            res
        }
    }
}

pub fn even_odd(nums: &[i32]) -> i64 {
    let [mut even, mut odd] = [0, 0];
    for &num in nums {
        let ev = even.max(odd + i64::from(num));
        let od = odd.max(even - i64::from(num));
        [even, odd] = [ev, od];
    }
    even
}

pub fn buy_sell_stock(nums: &[i32]) -> i64 {
    // Sell at high, buy at low
    nums.windows(2).map(|w| i64::from(w[1] - w[0]).max(0)).sum()
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
        assert_eq!(max_alternating_sum(&[4, 2, 5, 3]), 7);
        assert_eq!(max_alternating_sum(&[5, 6, 7, 8]), 8);
        assert_eq!(max_alternating_sum(&[6, 2, 1, 2, 4, 5]), 10);

        assert_eq!(even_odd(&[4, 2, 5, 3]), 7);
        assert_eq!(even_odd(&[5, 6, 7, 8]), 8);
        assert_eq!(even_odd(&[6, 2, 1, 2, 4, 5]), 10);
    }

    #[test]
    fn test() {}
}
