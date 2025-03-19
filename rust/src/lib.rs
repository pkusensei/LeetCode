mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_money(transactions: &[[i32; 2]]) -> i64 {
    let mut spend = 0;
    let [mut max_cost, mut max_cash] = [0, 0];
    for tr in transactions.iter() {
        let [cost, cash] = [0, 1].map(|i| i64::from(tr[i]));
        if cost > cash {
            // sum all deficits
            spend += cost - cash;
            // cashback cannot be used
            // start with max_cashback to start all transactions
            max_cash = max_cash.max(cash);
        } else {
            max_cost = max_cost.max(cost);
        }
    }
    spend + max_cost.max(max_cash)
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
        assert_eq!(minimum_money(&[[2, 1], [5, 0], [4, 2]]), 10);
        assert_eq!(minimum_money(&[[3, 0], [0, 3]]), 3);
    }

    #[test]
    fn test() {}
}
