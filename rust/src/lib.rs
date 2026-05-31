mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sale_items(items: Vec<[i32; 2]>, mut budget: i32) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut min_price = i32::MAX;
    // Preprocessing and count factors
    let mut max_factor = 1;
    let mut factors = HashMap::new();
    for item in items.iter() {
        let [fac, p] = item[..] else { unreachable!() };
        *factors.entry(fac).or_insert(0) += 1;
        min_price = min_price.min(p);
        max_factor = max_factor.max(fac);
    }
    // For each factor, add count(factor_multiple)
    let mut freq = HashMap::new();
    for &fac in factors.keys() {
        for p in (fac..=max_factor).step_by(fac as usize) {
            *freq.entry(fac).or_insert(0) += factors.get(&p).unwrap_or(&0);
        }
    }
    // Each item contributes 2*min(budget/price, freq[factor]-1)
    // budget/price => most amount to buy
    // freq[factor]-1 => possible free copies
    //      -1 to remove item itself
    // filter() remove item too expensive or non-existent
    let arr = items
        .iter()
        .map(|v| (v[1], freq[&v[0]] - 1))
        .filter(|v| v.0 < 2 * min_price && v.1 > 0)
        .sorted_unstable();
    let mut res = 0;
    for item in arr {
        let (p, f) = item;
        let curr = f.min(budget / p);
        res += 2 * curr;
        budget -= curr * p;
    }
    res + budget / min_price
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
    fn basics() {
        assert_eq!(maximum_sale_items(vec![[1, 6], [2, 4], [3, 5]], 19), 5);
        assert_eq!(
            maximum_sale_items(vec![[2, 8], [1, 10], [6, 6], [4, 12], [5, 20], [5, 17]], 35),
            7
        )
    }

    #[test]
    fn test() {}
}
