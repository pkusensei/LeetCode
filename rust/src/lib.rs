mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distance(nums: Vec<i32>) -> Vec<i64> {
    use std::collections::HashMap;
    let n = nums.len();
    let mut map = HashMap::<_, Vec<_>>::new();
    for (i, &num) in nums.iter().enumerate() {
        map.entry(num).or_default().push(i);
    }
    let mut res = vec![0; n];
    for ids in map.into_values() {
        if ids.len() <= 1 {
            continue;
        }
        let pref = ids.iter().fold(vec![], |mut acc, v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
        for (pos, i) in ids.into_iter().enumerate() {
            let left = if pos > 0 { pref[pos - 1] } else { 0 };
            let right = pref.last().unwrap() - pref[pos];
            let val = i * pos - left + right - (pref.len() - pos - 1) * i;
            res[i] = val as i64;
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
