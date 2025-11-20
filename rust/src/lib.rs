mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut heap: BinaryHeap<_> = arr[1..]
        .iter()
        .map(|&den| (Reverse(Frac { nom: 1, den }), 0))
        .collect();
    let mut nom = 0;
    let mut den = 0;
    for _ in 0..k {
        let Some((Reverse(curr), idx)) = heap.pop() else {
            break;
        };
        nom = curr.nom;
        den = curr.den;
        if let Some(v) = arr.get(1 + idx)
            && *v < den
        {
            heap.push((Reverse(Frac { nom: *v, ..curr }), 1 + idx));
        }
    }
    vec![nom, den]
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Frac {
    nom: i32,
    den: i32,
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Frac {
    // a/b - c/d = (ad-bc)/bd
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.nom * other.den).cmp(&(self.den * other.nom))
    }
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
