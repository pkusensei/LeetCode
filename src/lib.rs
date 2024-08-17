mod helper;

#[allow(unused_imports)]
use helper::*;

// https://hashnode.com/post/best-time-to-buy-and-sell-stocks-a-comprehensive-guide-cko8xg43m0oz194s114kz71pn
pub fn max_profit(k: i32, prices: &[i32]) -> i32 {
    let mut profit_after_buy = vec![i32::MIN; k as usize + 1];
    let mut profit_after_sell = vec![0; k as usize + 1];

    for &num in prices.iter() {
        for j in (1..=k as usize).rev() {
            profit_after_buy[j] = profit_after_buy[j].max(profit_after_sell[j - 1] - num);
            profit_after_sell[j] = profit_after_sell[j].max(profit_after_buy[j] + num);
        }
    }
    profit_after_sell[k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(2, &[2, 4, 1]), 2);
        debug_assert_eq!(max_profit(2, &[3, 2, 6, 5, 0, 3]), 7);
    }

    #[test]
    fn test() {}
}
