mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    let mut freq_count = HashMap::new();
    let mut max_freq = 0;
    for &num in &nums {
        let f = freq.entry(num).or_insert(0);
        *f += 1;
        *freq_count.entry(*f).or_insert(0) += 1;
        if *f > max_freq {
            max_freq = *f;
        }
    }
    max_freq * freq_count[&max_freq]
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
    fn basics() {}

    #[test]
    fn test() {}
}
