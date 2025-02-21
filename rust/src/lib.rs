mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_pairs(nums: &[&str], target: &str) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut res = 0;
    for s in nums.iter() {
        if let Some(suf) = target.strip_prefix(s).and_then(|suffix| map.get(suffix)) {
            res += suf;
        }
        if let Some(pre) = target.strip_suffix(s).and_then(|prefix| map.get(prefix)) {
            res += pre;
        }
        *map.entry(*s).or_insert(0) += 1;
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
        assert_eq!(num_of_pairs(&["777", "7", "77", "77"], "7777"), 4);
        assert_eq!(num_of_pairs(&["123", "4", "12", "34"], "1234"), 2);
        assert_eq!(num_of_pairs(&["1", "1", "1"], "11"), 6);
    }

    #[test]
    fn test() {}
}
