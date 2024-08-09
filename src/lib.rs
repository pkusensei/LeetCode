pub fn max_profit(prices: &[i32]) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    prices
        .windows(2)
        .filter_map(|w| if w[0] < w[1] { Some(w[1] - w[0]) } else { None })
        .sum()
    // for all price pairs
    // buy at low and sell at high
    // since sell-then-buy can happen on same day
    // this effectively means buy-hold-sell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 7);
        debug_assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
        debug_assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test() {}
}
