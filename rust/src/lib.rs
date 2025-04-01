mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(start: &[i32], target: &[i32], special_roads: &[[i32; 5]]) -> i32 {
    let [sx, sy] = [start[0], start[1]];
    let [tx, ty] = [target[0], target[1]];
    let mut dists = HashMap::new();
    dists.insert([sx, sy], 0);
    dists.insert([tx, ty], sx.abs_diff(tx) + sy.abs_diff(ty));
    let mut roads = HashMap::<_, HashMap<_, _>>::new();
    let mut heap = BinaryHeap::from([(Reverse(0), sx, sy)]);
    let mut vertices = HashSet::from([[tx, ty]]);
    for r in special_roads.iter() {
        let [x1, y1, x2, y2, c] = r[..] else {
            unreachable!()
        };
        let v = roads
            .entry([x1, y1])
            .or_default()
            .entry([x2, y2])
            .or_insert(c);
        *v = (*v).min(c);
        vertices.extend([[x1, y1], [x2, y2]]);
    }
    while let Some((Reverse(dist), x, y)) = heap.pop() {
        if dists.get(&[x, y]).is_some_and(|&v| v < dist) {
            continue;
        }
        if x == tx && y == ty {
            return dist as i32;
        }
        for vert in vertices.iter() {
            let mut d = dist + x.abs_diff(vert[0]) + y.abs_diff(vert[1]);
            if let Some(&c) = roads.get(&[x, y]).and_then(|v| v.get(vert)) {
                d = d.min(dist + c as u32);
            }
            if dists.get(vert).is_none_or(|&v| v > d) {
                dists.insert(*vert, d);
                heap.push((Reverse(d), vert[0], vert[1]));
            }
        }
    }
    dists[&[tx, ty]] as i32
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
        assert_eq!(
            minimum_cost(&[1, 1], &[4, 5], &[[1, 2, 3, 3, 2], [3, 4, 4, 5, 1]],),
            5
        );
        assert_eq!(
            minimum_cost(
                &[3, 2],
                &[5, 7],
                &[
                    [5, 7, 3, 2, 1],
                    [3, 2, 3, 4, 4],
                    [3, 3, 5, 5, 5],
                    [3, 4, 5, 6, 6]
                ]
            ),
            7
        );
        assert_eq!(
            minimum_cost(
                &[1, 1],
                &[10, 4],
                &[
                    [4, 2, 1, 1, 3],
                    [1, 2, 7, 4, 4],
                    [10, 3, 6, 1, 2],
                    [6, 1, 1, 2, 3]
                ]
            ),
            8
        );
    }

    #[test]
    fn test() {}
}
