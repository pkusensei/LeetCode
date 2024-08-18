mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn num_islands(grid: &[&[char]]) -> i32 {
    let mut seen = HashSet::new();
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '1' && seen.insert((x, y)) {
                find_cluster((x, y), &mut seen, grid);
                res += 1
            }
        }
    }
    res
}

fn find_cluster(coord: Coord, seen: &mut HashSet<Coord>, grid: &[&[char]]) {
    let mut queue = VecDeque::from([coord]);
    while let Some(coord) = queue.pop_front() {
        for (x, y) in neighbors(coord) {
            if grid
                .get(y)
                .is_some_and(|r| r.get(x).is_some_and(|&c| c == '1'))
                && seen.insert((x, y))
            {
                queue.push_back((x, y))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_islands(&[
                &['1', '1', '1', '1', '0'],
                &['1', '1', '0', '1', '0'],
                &['1', '1', '0', '0', '0'],
                &['0', '0', '0', '0', '0'],
            ]),
            1
        );
        debug_assert_eq!(
            num_islands(&[
                &['1', '1', '0', '0', '0'],
                &['1', '1', '0', '0', '0'],
                &['0', '0', '1', '0', '0'],
                &['0', '0', '0', '1', '1'],
            ]),
            3
        );
    }

    #[test]
    fn test() {}
}
