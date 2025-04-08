mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_seconds(nums: &[i32]) -> i32 {
    use std::collections::HashMap;
    let n = nums.len() as i32;
    let mut map = HashMap::<_, Vec<_>>::new();
    for (i, &num) in nums.iter().enumerate() {
        map.entry(num).or_default().push(i as i32);
    }
    let mut res = n;
    for v in map.values() {
        if v.len() == 1 {
            res = res.min(n / 2);
        } else {
            let curr: i32 = v.windows(2).map(|w| (w[1] - w[0]) / 2).max().unwrap();
            res = res.min(curr.max((v[0] + n - v.last().unwrap()) / 2));
        }
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
        assert_eq!(minimum_seconds(&[1, 2, 1, 2]), 1);
        assert_eq!(minimum_seconds(&[2, 1, 3, 3, 2]), 2);
        assert_eq!(minimum_seconds(&[5, 5, 5, 5]), 0);
    }

    #[test]
    fn test() {}
}
