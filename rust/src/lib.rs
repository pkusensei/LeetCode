mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
    use std::collections::HashMap;

    let mut factors = HashMap::new();
    let mut max_factor = 1;
    let mut min_price = i32::MAX;
    for item in items.iter() {
        let [factor, price] = item[..] else {
            unreachable!()
        };
        *factors.entry(factor).or_insert(0) += 1;
        max_factor = max_factor.max(factor);
        min_price = min_price.min(price);
    }
    let mut freq = HashMap::new();
    for &factor in factors.keys() {
        for f in (factor..=max_factor).step_by(factor as usize) {
            *freq.entry(factor).or_insert(0) += factors.get(&f).unwrap_or(&0);
        }
    }
    let mut to_buy: Vec<_> = items
        .iter()
        .map(|v| (v[1], freq[&v[0]] - 1))
        .filter(|&(price, free)| price < 2 * min_price && free > 0)
        .collect();
    to_buy.sort_unstable();
    let mut res = 0;
    let mut budget = budget;
    for (price, free) in to_buy {
        let curr = free.min(budget / price);
        res += 2 * curr;
        budget -= curr * price;
    }
    res + budget / min_price
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
