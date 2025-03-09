mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut res: Vec<_> = nums
        .into_iter()
        .flatten()
        .fold(std::collections::HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(i, v)| if v == n { Some(i) } else { None })
        .collect();
    res.sort_unstable();
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
    fn basics() {}

    #[test]
    fn test() {}
}
