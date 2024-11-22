mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn is_escape_possible(blocked: &[[i32; 2]], source: [i32; 2], target: [i32; 2]) -> bool {
    let blocks: HashSet<_> = blocked.iter().map(|v| [v[0], v[1]]).collect();
    bfs(source, target, &blocks) && bfs(target, source, &blocks)
    // dfs(source, target, &blocks, &mut HashSet::new())
    //     && dfs(target, source, &blocks, &mut HashSet::new())
}

const LIMIT: i32 = 1_000_000;

fn bfs(source: [i32; 2], target: [i32; 2], blocks: &HashSet<[i32; 2]>) -> bool {
    let mut queue = VecDeque::from([source]);
    let mut seen = HashSet::from([source]);
    while let Some([x, y]) = queue.pop_front() {
        for next in [[0, 1], [0, -1], [1, 0], [-1, 0]].map(|[dx, dy]| [dx + x, dy + y]) {
            if next.iter().any(|v| !(0..LIMIT).contains(v)) {
                continue;
            }
            if blocks.contains(&next) || !seen.insert(next) {
                continue;
            }
            if next == target {
                return true;
            }
            queue.push_back(next);
        }
        if seen.len() == 20_000 {
            return true;
        }
    }
    false
}

// stack overflow
fn dfs(
    source: [i32; 2],
    target: [i32; 2],
    blocks: &HashSet<[i32; 2]>,
    seen: &mut HashSet<[i32; 2]>,
) -> bool {
    if source.iter().any(|v| !(0..LIMIT).contains(v)) {
        return false;
    }
    if blocks.contains(&source) || !seen.insert(source) {
        return false;
    }
    if source == target || seen.len() > 20_000 {
        return true;
    }
    let [x, y] = source;
    dfs([x - 1, y], target, blocks, seen)
        || dfs([x + 1, y], target, blocks, seen)
        || dfs([x, y - 1], target, blocks, seen)
        || dfs([x, y + 1], target, blocks, seen)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!is_escape_possible(&[[0, 1], [1, 0]], [0, 0], [0, 2]));
        debug_assert!(is_escape_possible(&[], [0, 0], [999999, 999999]));
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
