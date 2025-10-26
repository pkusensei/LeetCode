mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_stable_subarrays(capacity: &[i32]) -> i64 {
    use std::collections::HashMap;
    let n = capacity.len();
    let mut num_ids = HashMap::<_, Vec<_>>::new();
    let mut prefix = Vec::with_capacity(n);
    for (i, &v) in capacity.iter().enumerate() {
        num_ids.entry(i64::from(v)).or_default().push(i);
        prefix.push(i64::from(v) + prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    for (num, ids) in num_ids {
        let mut map = HashMap::<_, Vec<_>>::new();
        for i in ids {
            if let Some(v) = map.get(&(prefix[i] - 2 * num)) {
                res += v.len() as i64;
                if v.last().is_some_and(|&prev| prev + 1 == i) {
                    res -= 1;
                }
            }
            map.entry(prefix[i]).or_default().push(i);
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
    fn basics() {
        assert_eq!(count_stable_subarrays(&[9, 3, 3, 3, 9]), 2);
        assert_eq!(count_stable_subarrays(&[1, 2, 3, 4, 5]), 0);
        assert_eq!(count_stable_subarrays(&[-4, 4, 0, 0, -8, -4]), 1);
    }

    #[test]
    fn test() {}
}
