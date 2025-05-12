mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn non_special_count(l: i32, r: i32) -> i32 {
    let [root1, root2] = [l, r].map(|v| v.isqrt());
    let mut sieve = vec![true; 1 + root2 as usize];
    sieve[..2].fill(false);
    for p in 2..=root2 as usize {
        if sieve[p] {
            for val in (p * p..=root2 as usize).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    let mut special = (root1..=root2).filter(|&v| sieve[v as usize]).count();
    if root1.pow(2) < l && sieve[root1 as usize] {
        special = special.saturating_sub(1);
    }
    r - l + 1 - special as i32
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
        assert_eq!(non_special_count(4, 16), 11);
    }

    #[test]
    fn test() {
        assert_eq!(non_special_count(5, 25), 19);
        assert_eq!(non_special_count(1, 4), 3);
    }
}
