mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_good_partitions(nums: &[i32]) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (idx, &num) in nums.iter().enumerate() {
        let v = map.entry(num).or_insert([idx, idx]);
        v[1] = idx;
    }
    let spans = map.into_values().sorted_unstable().collect_vec();
    let mut count = 1;
    let mut end = spans[0][1];
    for &[s, e] in &spans[1..] {
        count += i64::from(s > end);
        end = end.max(e);
    }
    mod_pow(2, count - 1, 1_000_000_007) as i32
}

const fn mod_pow(b: i64, e: i64, m: i64) -> i64 {
    if e == 0 {
        return 1;
    }
    if e & 1 == 0 {
        mod_pow(b * b % m, e >> 1, m)
    } else {
        mod_pow(b * b % m, e >> 1, m) * b % m
    }
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
        assert_eq!(number_of_good_partitions(&[1, 2, 3, 4]), 8);
        assert_eq!(number_of_good_partitions(&[1, 1, 1, 1]), 1);
        assert_eq!(number_of_good_partitions(&[1, 2, 1, 3]), 2);
    }

    #[test]
    fn test() {}
}
