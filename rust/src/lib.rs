mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_min_height_trees(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let mut n = n as usize;
    let mut graph: HashMap<i32, Vec<_>> = edges.iter().fold(HashMap::new(), |mut acc, e| {
        acc.entry(e[0]).or_default().push(e[1]);
        acc.entry(e[1]).or_default().push(e[0]);
        acc
    });
    let mut leaf: Vec<_> = graph
        .iter()
        .filter_map(|(&k, v)| if v.len() == 1 { Some(k) } else { None })
        .collect();
    while n > 2 {
        n -= leaf.len();
        let mut new_leaf = vec![];
        for node in leaf {
            let neighbor = graph[&node][0];
            if let Some(v) = graph.get_mut(&neighbor) {
                v.retain(|&num| num != node);
                if v.len() == 1 {
                    new_leaf.push(neighbor)
                }
            }
        }
        leaf = new_leaf;
    }
    leaf
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_min_height_trees(4, &[[1, 0], [1, 2], [1, 3]]), [1]);
        sort_eq(
            find_min_height_trees(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]),
            [3, 4],
        );
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
