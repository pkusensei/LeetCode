mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut map = BTreeMap::new();
    for item in items.iter() {
        let [key, val] = item[..] else { unreachable!() };
        let v = map.entry(key).or_insert(val);
        *v = (*v).max(val);
    }
    let mut val = 0;
    for v in map.values_mut() {
        val = val.max(*v);
        *v = val;
    }
    let mut res = vec![];
    let mut seen = HashMap::new();
    for &q in queries.iter() {
        if let Some(v) = seen.get(&q) {
            res.push(*v);
        } else {
            let v = map.range(..=q).next_back().map(|(_, v)| *v).unwrap_or(0);
            res.push(v);
            seen.insert(q, v);
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
    fn basics() {}

    #[test]
    fn test() {}
}
