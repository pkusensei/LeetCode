mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(prices: &[i32], strategy: &[i32], k: i32) -> i64 {
    use itertools::izip;
    let k = k as usize;
    let sum: i64 = izip!(prices.iter(), strategy.iter())
        .map(|(p, s)| i64::from(p * s))
        .sum();
    let mut res = sum;
    let mut curr = 0;
    // The first window of k
    for i in 0..k / 2 {
        // "lose" these k/2 values
        curr += i64::from(prices[i] * (0 - strategy[i]));
    }
    for i in k / 2..k {
        // "gain" k/2 values
        curr += i64::from(prices[i] * (1 - strategy[i]));
    }
    res = res.max(curr + sum);
    // For each window
    for (idx, (&pr, &st)) in izip!(prices.iter(), strategy.iter()).enumerate().skip(k) {
        curr += i64::from(pr * (1 - st)); // "gain" this value
        curr -= i64::from(prices[idx - k / 2]); // "Neutralize" mid value
        let prev = idx - k;
        curr += i64::from(prices[prev] * strategy[prev]); // "Restore" dropped value
        res = res.max(sum + curr);
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
        assert_eq!(max_profit(&[5, 4, 3], &[1, 1, 0], 2), 9);
    }

    #[test]
    fn test() {}
}
