mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(m: i32, n: i32, wait_cost: &[&[i32]]) -> i64 {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };
    let mut heap = BinaryHeap::from([(Reverse(1), 0, 0, true)]);
    let mut dists = HashMap::from([((0, 0, true), 1)]);
    while let Some((Reverse(cost), r, c, odd)) = heap.pop() {
        if r == m - 1 && c == n - 1 {
            return cost;
        }
        if dists.get(&(r, c, odd)).is_some_and(|&v| v < cost) {
            continue;
        }
        if odd {
            for [nr, nc] in [[r + 1, c], [r, 1 + c]]
                .into_iter()
                .filter(|&[nr, nc]| nr < m && nc < n)
            {
                let ncost = cost + i64::from(1 + nr) * i64::from(1 + nc);
                if dists.get(&(nr, nc, false)).is_none_or(|&v| v < ncost) {
                    dists.insert((nr, nc, false), ncost);
                    heap.push((Reverse(ncost), nr, nc, false));
                }
            }
        } else {
            let ncost = cost + i64::from(wait_cost[r as usize][c as usize]);
            if dists.get(&(r, c, true)).is_none_or(|&v| v > ncost) {
                dists.insert((r, c, true), ncost);
                heap.push((Reverse(ncost), r, c, true));
            }
        }
    }
    -1
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
        assert_eq!(min_cost(2, 2, &[&[3, 5], &[2, 4]]), 9);
        assert_eq!(min_cost(2, 3, &[&[6, 1, 4], &[3, 2, 5]]), 16);
    }

    #[test]
    fn test() {}
}
