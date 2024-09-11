mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
    // bfs(x, y, target, 0, 0)

    // if x and y are nonzero integers and g = gcd(x,y),
    // then there exist integers a and b such that ax+by=g
    x + y == target || (x + y > target && target % gcd(x, y) == 0)
}

fn bfs(x: i32, y: i32, target: i32, jx: i32, jy: i32) -> bool {
    let mut queue = VecDeque::from([(jx, jy)]);
    let mut seen = HashSet::new();
    while let Some((cx, cy)) = queue.pop_front() {
        if !seen.insert((cx, cy)) {
            continue;
        }
        if cx.abs_diff(cy) == target as u32 || cx + cy == target {
            return true;
        }
        queue.extend(next(x, y, cx, cy))
    }
    false
}

fn next(x: i32, y: i32, jx: i32, jy: i32) -> Vec<(i32, i32)> {
    let mut res = vec![(x, jy), (jx, y), (0, jy), (jx, 0)];
    let (dx, dy) = (x - jx, y - jy);
    // x -> y
    if jx > dy {
        res.push((jx - dy, y));
    } else {
        res.push((0, jy + jx));
    }
    // y -> x
    if jy > dx {
        res.push((x, jy - dx));
    } else {
        res.push((jx + jy, 0));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_measure_water(3, 5, 4));
        debug_assert!(!can_measure_water(2, 6, 5));
        debug_assert!(can_measure_water(1, 2, 3));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
