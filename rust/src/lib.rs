mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn complete_prime(num: i32) -> bool {
    let s = num.to_string();
    let n = s.len();
    for i in 0..n {
        if let Ok(pref) = s[..i].parse::<i32>()
            && !is_prime(pref)
        {
            return false;
        }
        if let Ok(suf) = s[i..].parse::<i32>()
            && !is_prime(suf)
        {
            return false;
        }
    }
    true
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    for &p in S.iter() {
        if num < p {
            break;
        }
        if num % p == 0 && num > p {
            return false;
        }
    }
    true
}

static S: LazyLock<Vec<i32>> = LazyLock::new(|| {
    const MAX: usize = 31_623; // or some big number
    let mut res = vec![true; 1 + MAX]; // assume all are prime
    res[..2].fill(false); // 0 and 1 are not prime
    res[2] = true;
    for p in 2..=MAX {
        if res[p] {
            // For this prime, all of its multiples are non-prime
            for val in (p * p..=MAX).step_by(p) {
                res[val] = false;
            }
        }
    }
    res.into_iter()
        .enumerate()
        .filter_map(|(p, v)| if v { Some(p as i32) } else { None })
        .collect()
});

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
        assert!(!complete_prime(39));
    }

    #[test]
    fn test() {}
}
