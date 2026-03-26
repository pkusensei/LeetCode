mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    let [mut pos, mut neg] = [HashMap::new(), HashMap::new()];
    let mut zero = 0;
    for num in arr {
        match num.cmp(&0) {
            std::cmp::Ordering::Less => *neg.entry(-num).or_insert(0) += 1,
            std::cmp::Ordering::Equal => zero += 1,
            std::cmp::Ordering::Greater => *pos.entry(num).or_insert(0) += 1,
        }
    }
    zero & 1 == 0 && f(pos) && f(neg)
}

fn f(mut map: HashMap<i32, i32>) -> bool {
    let nums = map.keys().copied().sorted_unstable().collect_vec();
    for num in nums {
        let Some(f) = map.remove(&num) else {
            return false;
        };
        if f == 0 {
            continue;
        }
        let Some(big) = map.get_mut(&(2 * num)) else {
            return false;
        };
        *big -= f;
        if *big < 0 {
            return false;
        }
    }
    true
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
