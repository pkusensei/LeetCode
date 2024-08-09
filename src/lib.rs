pub fn max_profit(prices: &[i32]) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut buy = prices[0];
    let mut profit = 0;
    for &num in &prices[1..] {
        // save current minimum
        buy = buy.min(num);
        // for all bigger numbers on the right
        profit = profit.max(num - buy);
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
        debug_assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test() {}
}
