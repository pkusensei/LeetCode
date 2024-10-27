mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn largest_overlap(img1: &[&[i32]], img2: &[&[i32]]) -> i32 {
    let n = img1.len();
    let mut source = vec![];
    let mut target = HashSet::new();
    for y in 0..n {
        for x in 0..n {
            if img1[y][x] == 1 {
                source.push([x as i32, y as i32]);
            }
            if img2[y][x] == 1 {
                target.insert([x as i32, y as i32]);
            }
        }
    }
    let mut moves = HashMap::new();
    for &[x1, y1] in source.iter() {
        for &[x2, y2] in target.iter() {
            let t = [x2 - x1, y2 - y1];
            moves.entry(t).or_insert_with(|| count(t, &source, &target));
        }
    }
    moves.into_values().max().unwrap_or(0)
}

fn count(t: [i32; 2], source: &[[i32; 2]], target: &HashSet<[i32; 2]>) -> i32 {
    source
        .iter()
        .map(|v| [v[0] + t[0], v[1] + t[1]])
        .filter(|v| target.contains(v))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            largest_overlap(
                &[&[1, 1, 0], &[0, 1, 0], &[0, 1, 0]],
                &[&[0, 0, 0], &[0, 1, 1], &[0, 0, 1]]
            ),
            3
        );
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
