mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn all_paths_source_target(graph: &[&[i32]]) -> Vec<Vec<i32>> {
    let n = graph.len() as i32;
    let mut queue = std::collections::VecDeque::from([vec![0]]);
    let mut res = vec![];
    while let Some(path) = queue.pop_front() {
        let Some(&last) = path.last() else {
            break;
        };
        if last == n - 1 {
            res.push(path);
            continue;
        }
        for &next in graph[last as usize].iter() {
            if !path.contains(&next) {
                let mut p = path.clone();
                p.push(next);
                queue.push_back(p);
            }
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
        sort_eq(
            all_paths_source_target(&[&[1, 2], &[3], &[3], &[]]),
            [vec![0, 1, 3], vec![0, 2, 3]],
        );
        sort_eq(
            all_paths_source_target(&[&[4, 3, 1], &[3, 2, 4], &[3], &[4], &[]]),
            [
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4],
            ],
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
