mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn distance(nums: &[i32]) -> Vec<i64> {
    let prefs =
        (0..)
            .zip(nums.iter())
            .fold(HashMap::<_, Vec<[i64; 2]>>::new(), |mut acc, (i, &num)| {
                let v = acc.entry(num).or_default();
                let last = v.last().map(|[_, p]| p).unwrap_or(&0);
                v.push([i, i + last]);
                acc
            });
    let mut res = vec![];
    for (idx, num) in (0..).zip(nums.iter()) {
        let Some(prefix) = prefs.get(num) else {
            unreachable!()
        };
        if prefix.len() <= 1 {
            res.push(0);
            continue;
        }
        let Ok(i) = prefix.binary_search_by_key(&idx, |&[i, _]| i) else {
            unreachable!()
        };
        let left = if i > 0 { prefix[i - 1][1] } else { 0 };
        let right = prefix.last().unwrap()[1] - prefix[i][1];
        let len = prefix.len() as i64;
        res.push(i as i64 * idx - left + right - (len - 1 - i as i64) * idx);
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
        assert_eq!(distance(&[1, 3, 1, 1, 2]), [5, 0, 3, 4, 0]);
        assert_eq!(distance(&[0, 5, 3]), [0, 0, 0]);
    }

    #[test]
    fn test() {}
}
