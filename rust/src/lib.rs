mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn island_perimeter(grid: &[&[i32]]) -> i32 {
    let mut res = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &n) in row.iter().enumerate() {
            if n == 1 {
                queue.push_back((x, y));
                while let Some((x, y)) = queue.pop_front() {
                    if !seen.insert((x, y)) {
                        continue;
                    }
                    res += 4;
                    for neighbor in neighbors((x, y)).filter(|&(nx, ny)| {
                        grid.get(ny)
                            .is_some_and(|r| r.get(nx).is_some_and(|&c| c == 1))
                    }) {
                        queue.push_back(neighbor);
                        res -= 1;
                    }
                }
                break;
            }
        }
        if res > 0 {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            island_perimeter(&[&[0, 1, 0, 0], &[1, 1, 1, 0], &[0, 1, 0, 0], &[1, 1, 0, 0]]),
            16
        );
        debug_assert_eq!(island_perimeter(&[&[1]]), 4);
        debug_assert_eq!(island_perimeter(&[&[1, 0]]), 4);
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
