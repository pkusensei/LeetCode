pub fn max_profit(prices: &[i32]) -> i32 {
    let size = prices.len();
    let mut dp = vec![vec![vec![-1; 3]; 2]; size];
    solve(0, 1, 2, prices, &mut dp)
}

fn solve(
    idx: usize,
    buy: usize,
    transaction: usize,
    prices: &[i32],
    dp: &mut [Vec<Vec<i32>>],
) -> i32 {
    if idx == prices.len() || transaction == 0 {
        return 0;
    }
    if dp[idx][buy][transaction] != -1 {
        return dp[idx][buy][transaction];
    }

    let profit = if buy == 1 {
        // no buy on idx
        let a = solve(idx + 1, 1, transaction, prices, dp);
        // buy on idx
        let b = solve(idx + 1, 0, transaction, prices, dp) - prices[idx];
        a.max(b)
    } else {
        // no sell on idx
        let a = solve(idx + 1, 0, transaction, prices, dp);
        // sell on idx
        let b = solve(idx + 1, 1, transaction - 1, prices, dp) + prices[idx];
        a.max(b)
    };
    dp[idx][buy][transaction] = profit;
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(&[3, 3, 5, 0, 0, 3, 1, 4]), 6);
        debug_assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
        debug_assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(max_profit(&[1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
    }
}
