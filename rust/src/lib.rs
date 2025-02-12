mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold(
            std::collections::HashMap::<_, Vec<_>>::new(),
            |mut acc, &n| {
                let mut curr = 0;
                let mut num = n;
                while num > 0 {
                    curr += num % 10;
                    num /= 10;
                }
                acc.entry(curr).or_default().push(n);
                acc
            },
        )
        .into_values()
        .filter_map(|mut v| {
            if v.len() > 1 {
                v.sort_unstable_by(|a, b| b.cmp(a));
                Some(v[0] + v[1])
            } else {
                None
            }
        })
        .max()
        .unwrap_or(-1)
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
