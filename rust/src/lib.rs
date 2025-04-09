mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_elegance(mut items: Vec<[i32; 2]>, k: i32) -> i64 {
    let k = k as usize;
    items.sort_unstable_by_key(|v| Reverse(v[0]));
    let mut profit = 0;
    let mut cats = HashMap::new();
    let mut heap = BinaryHeap::new();
    // Greedily sum up bigger profits
    for &v in items[..k].iter() {
        let [p, c] = v[..] else { unreachable!() };
        profit += i64::from(p);
        *cats.entry(c).or_insert(0) += 1;
        heap.push((Reverse(p), c));
    }
    let mut res = profit + (cats.len() as i64).pow(2);
    for item in items[k..].iter() {
        let [pro, cat] = item[..] else { unreachable!() };
        if cats.contains_key(&cat) {
            continue; // Only new cat wanted
        }
        while heap
            .peek()
            .is_some_and(|v| cats.get(&v.1).is_some_and(|&count| count == 1))
        {
            heap.pop(); // Pop all cats with count<2
        }
        let Some((Reverse(p), c)) = heap.pop() else {
            break;
        };
        profit += i64::from(pro - p);
        *cats.entry(cat).or_insert(0) += 1;
        let count = cats.entry(c).or_insert(0);
        *count -= 1;
        res = res.max(profit + (cats.len() as i64).pow(2));
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
        assert_eq!(find_maximum_elegance(vec![[3, 2], [5, 1], [10, 1]], 2), 17);
        assert_eq!(
            find_maximum_elegance(vec![[3, 1], [3, 1], [2, 2], [5, 3]], 3),
            19
        );
        assert_eq!(find_maximum_elegance(vec![[1, 1], [2, 1], [3, 1]], 3), 7);
    }

    #[test]
    fn test() {
        assert_eq!(
            find_maximum_elegance(vec![[6, 2], [10, 3], [7, 3], [7, 1]], 2),
            21
        );
        assert_eq!(
            find_maximum_elegance(vec![[3, 4], [8, 4], [2, 2], [1, 3]], 2),
            14
        );
    }
}
