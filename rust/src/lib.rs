mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_fair_pairs(nums: &[i32], lower: i32, upper: i32) -> i64 {
    let mut map = std::collections::BTreeMap::new();
    let mut res = 0;
    for &num in nums.iter() {
        res += map
            .range(lower - num..=upper - num)
            .map(|(_, v)| *v as i64)
            .sum::<i64>();
        *map.entry(num).or_insert(0) += 1;
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
        assert_eq!(count_fair_pairs(&[0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(count_fair_pairs(&[1, 7, 9, 2, 5], 11, 11), 1);
    }

    #[test]
    fn test() {}
}
