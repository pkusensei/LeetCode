mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mut seen = std::collections::HashMap::new();
    let p2s: Vec<_> = (0..=21).map(|p| 2i32.pow(p)).collect();
    let mut res = 0;
    for num in deliciousness {
        for p in p2s.iter() {
            if let Some(v) = seen.get(&(p - num)) {
                res += v;
                res %= 1_000_000_007;
            }
        }
        *seen.entry(num).or_insert(0) += 1;
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
