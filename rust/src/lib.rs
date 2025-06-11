mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_same_digits(s: &str) -> bool {
    let n = s.len();
    let [mut s1, mut s2] = [0, 0];
    for (idx, w) in s.as_bytes().windows(2).enumerate() {
        let val = bin_mod10(n - 2, idx); // C(n-2, idx) %10
        let [d1, d2] = [0, 1].map(|i| usize::from(w[i] - b'0'));
        s1 = (s1 + val * d1) % 10;
        s2 = (s2 + val * d2) % 10;
    }
    s1 == s2
}

fn bin_mod10(n: usize, k: usize) -> usize {
    let mod2 = bin_mod2(n, k);
    let mod5 = bin_mod5(n, k);
    (0..10)
        .find(|&i| i % 2 == mod2 && i % 5 == mod5)
        .unwrap_or(0)
}

const fn bin_mod2(mut n: usize, mut k: usize) -> usize {
    while k > 0 {
        if k & 1 > n & 1 {
            return 0;
        }
        n >>= 1;
        k >>= 1;
    }
    1
}

fn bin_mod5(mut n: usize, mut k: usize) -> usize {
    let mut res = 1;
    while n > 0 || k > 0 {
        let nd = n % 5;
        let kd = k % 5;
        if kd > nd {
            return 0;
        }
        res = res * bin_small(nd, kd) % 5;
        n /= 5;
        k /= 5;
    }
    res
}

fn bin_small(n: usize, k: usize) -> usize {
    // [0!, 1!, 2!, 3!, 4!] %5
    const FACT: [usize; 5] = [1, 1, 2, 1, 4];
    if k > n {
        return 0;
    }
    let numerator = FACT[n];
    let denominator = FACT[k] * FACT[n - k] % 5;
    let den_inv = (0..5).find(|&i| (denominator * i) % 5 == 1).unwrap_or(0);
    numerator * den_inv % 5
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
        assert!(has_same_digits("3902"));
        assert!(!has_same_digits("34789"));
    }

    #[test]
    fn test() {}
}
