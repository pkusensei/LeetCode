mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    use itertools::izip;
    use std::{
        cmp::Reverse,
        collections::{BTreeMap, BinaryHeap},
    };
    // server - (time, count)
    let mut free: BTreeMap<_, _> = (0..k).map(|i| (i, (0, 0))).collect();
    let mut busy: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
    let mut max = 0;
    for (i, (&arr, &loa)) in izip!(arrival.iter(), load.iter()).enumerate() {
        while let Some(&(Reverse(time), server, count)) = busy.peek()
            && time <= arr
        {
            free.insert(server, (time, count));
            busy.pop();
        }
        let Some((&server, &(_time, count))) = free
            .range(i as i32 % k..)
            .next()
            .or_else(|| free.iter().next())
        else {
            continue; // empty
        };
        free.remove(&server);
        max = max.max(1 + count);
        busy.push((Reverse(arr + loa), server, 1 + count));
    }
    while let Some((Reverse(time), server, count)) = busy.pop() {
        free.insert(server, (time, count));
        max = max.max(count);
    }
    free.into_iter()
        .filter_map(|(server, (_, count))| if count == max { Some(server) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
