mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn pacific_atlantic(heights: &[&[i32]]) -> Vec<[i32; 2]> {
    let (rows, cols) = get_dimensions(heights);
    let (mut p_set, mut a_set) = (HashSet::new(), HashSet::new());
    for x in 0..cols {
        bfs(heights, (x, 0), &mut p_set);
        bfs(heights, (x, rows - 1), &mut a_set);
    }
    for y in 0..rows {
        bfs(heights, (0, y), &mut p_set);
        bfs(heights, (cols - 1, y), &mut a_set);
    }
    p_set
        .intersection(&a_set)
        .map(|c| [c.1 as i32, c.0 as i32])
        .collect()
}

fn bfs(heights: &[&[i32]], curr: Coord, seen: &mut HashSet<Coord>) {
    if seen.contains(&curr) {
        return;
    }
    let mut queue = VecDeque::from([curr]);
    while let Some(c) = queue.pop_front() {
        if !seen.insert(c) {
            continue;
        }
        for (nx, ny) in
            neighbors(c).filter(|&(x, y)| heights.get(y).is_some_and(|r| r.get(x).is_some()))
        {
            if heights[ny][nx] >= heights[c.1][c.0] {
                queue.push_back((nx, ny));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            pacific_atlantic(&[
                &[1, 2, 2, 3, 5],
                &[3, 2, 3, 4, 4],
                &[2, 4, 5, 3, 1],
                &[6, 7, 1, 4, 5],
                &[5, 1, 1, 2, 4],
            ]),
            [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]],
        );
        debug_assert_eq!(pacific_atlantic(&[&[1]]), [[0, 0]]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
