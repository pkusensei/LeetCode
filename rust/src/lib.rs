mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn most_popular_creator(
    creators: Vec<String>,
    ids: Vec<String>,
    views: Vec<i32>,
) -> Vec<Vec<String>> {
    let mut map = HashMap::<_, (i64, BinaryHeap<_>)>::new();
    for ((cr, id), view) in creators.iter().zip(ids.iter()).zip(views.iter()) {
        let v = map.entry(cr.as_str()).or_default();
        v.0 += i64::from(*view);
        v.1.push((view, Reverse(id.as_str())));
    }
    let mut max_score = 0;
    let mut res = vec![];
    for (cr, (score, mut heap)) in map.into_iter() {
        if score > max_score {
            max_score = score;
            res.clear();
        }
        if score == max_score {
            let id = heap.pop().unwrap().1.0.to_string();
            res.push(vec![cr.to_string(), id]);
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
