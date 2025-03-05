mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    nums.windows(2)
        .filter_map(|w| if w[0] == key { Some(w[1]) } else { None })
        .fold(std::collections::HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_k, v)| v)
        .map(|(k, _v)| k)
        .unwrap()
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
    fn test() {
        assert_eq!(most_frequent(vec![2, 1, 2, 1, 2, 3], 2), 1);
    }
}
