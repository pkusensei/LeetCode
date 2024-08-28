mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn count_sub_islands(grid1: &[&[i32]], grid2: &[&[i32]]) -> i32 {
    let mut seen = HashSet::new();
    let mut res = 0;

    for (y, row) in grid2.iter().enumerate() {
        for (x, &n) in row.iter().enumerate() {
            if n == 1
                && find_cluster((x, y), grid2, &mut seen)
                    .is_some_and(|cluster| cluster.into_iter().all(|(x, y)| grid1[y][x] == 1))
            {
                res += 1
            }
        }
    }
    res
}

fn find_cluster(
    coord: Coord,
    grid2: &[&[i32]],
    seen: &mut HashSet<Coord>,
) -> Option<HashSet<Coord>> {
    if !seen.insert(coord) {
        return None;
    }

    let mut queue = VecDeque::from([coord]);
    let mut res = HashSet::from([coord]);
    while let Some(c) = queue.pop_front() {
        for (x, y) in neighbors(c) {
            if grid2
                .get(y)
                .is_some_and(|r| r.get(x).is_some_and(|&n| n == 1))
                && seen.insert((x, y))
            {
                queue.push_back((x, y));
                res.insert((x, y));
            }
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            count_sub_islands(
                &[
                    &[1, 1, 1, 0, 0],
                    &[0, 1, 1, 1, 1],
                    &[0, 0, 0, 0, 0],
                    &[1, 0, 0, 0, 0],
                    &[1, 1, 0, 1, 1]
                ],
                &[
                    &[1, 1, 1, 0, 0],
                    &[0, 0, 1, 1, 1],
                    &[0, 1, 0, 0, 0],
                    &[1, 0, 1, 1, 0],
                    &[0, 1, 0, 1, 0]
                ]
            ),
            3
        );
        debug_assert_eq!(
            count_sub_islands(
                &[
                    &[1, 0, 1, 0, 1],
                    &[1, 1, 1, 1, 1],
                    &[0, 0, 0, 0, 0],
                    &[1, 1, 1, 1, 1],
                    &[1, 0, 1, 0, 1]
                ],
                &[
                    &[0, 0, 0, 0, 0],
                    &[1, 1, 1, 1, 1],
                    &[0, 1, 0, 1, 0],
                    &[0, 1, 0, 1, 0],
                    &[1, 0, 0, 0, 1]
                ]
            ),
            2
        );
    }

    #[test]
    fn test() {}
}
