mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(prices: &[i32], strategy: &[i32], k: i32) -> i64 {
    use itertools::izip;
    let n = prices.len();
    let mut pref_p = Vec::with_capacity(n);
    let mut pref_s = Vec::with_capacity(n);
    for (p, s) in izip!(prices, strategy) {
        pref_p.push(i64::from(*p) + pref_p.last().unwrap_or(&0));
        pref_s.push(i64::from(p * s) + pref_s.last().unwrap_or(&0));
    }
    let mut res = *pref_s.last().unwrap();
    let k = k as usize / 2;
    let mut curr = res - pref_s[2 * k - 1];
    curr += pref_p[2 * k - 1] - pref_p[k - 1];
    res = res.max(curr);
    for idx in 2 * k..n {
        curr += i64::from(prices[idx - 2 * k] * strategy[idx - 2 * k]);
        curr += i64::from(prices[idx] * (1 - strategy[idx]));
        curr -= i64::from(prices[idx - k]);
        res = res.max(curr);
    }
    res
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
        assert_eq!(max_profit(&[4, 2, 8], &[-1, 0, 1], 2), 10);
    }

    #[test]
    fn test() {
        assert_eq!(max_profit(&[4, 7, 13], &[-1, -1, 0], 2), 9);
    }
}
