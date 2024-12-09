mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_items(n: i32, m: i32, group: &mut [i32], before_items: &[&[i32]]) -> Vec<i32> {
    let [n, m] = [n, m].map(|v| v as usize);
    let mut indegs = vec![0; n];
    let mut adj = vec![vec![]; n];
    for (item, bef) in before_items.iter().enumerate() {
        for &b in bef.iter() {
            adj[b as usize].push(item);
            indegs[item] += 1;
        }
    }
    // Topo sort each item
    let item_ids = topo_sort(&adj, &mut indegs);
    if item_ids.len() != n {
        return vec![];
    }

    // Put each item into their groups now that they're in order
    let mut groups = vec![vec![]; m];
    for id in item_ids.into_iter() {
        if group[id] == -1 {
            group[id] = groups.len() as i32;
            groups.push(vec![]);
        }
        groups[group[id] as usize].push(id as i32);
    }
    let gn = groups.len();
    let mut indegs = vec![0; gn];
    let mut adj = vec![vec![]; gn];
    for (item, bef) in before_items.iter().enumerate() {
        for &b in bef.iter() {
            // source group, target group
            let (src, dst) = (group[b as usize] as usize, group[item] as usize);
            if src == dst {
                continue;
            }
            adj[src].push(dst);
            indegs[dst] += 1;
        }
    }
    // Topo sort groups
    let group_ids = topo_sort(&adj, &mut indegs);
    if group_ids.len() != gn {
        return vec![];
    }
    group_ids.into_iter().fold(vec![], |mut acc, i| {
        acc.append(&mut groups[i]);
        acc
    })
}

fn topo_sort(adj: &[Vec<usize>], indegs: &mut [i32]) -> Vec<usize> {
    let mut queue: std::collections::VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| if d == 0 { Some(i) } else { None })
        .collect();
    let mut res = Vec::with_capacity(indegs.len());
    while let Some(node) = queue.pop_front() {
        res.push(node);
        for &next in adj[node].iter() {
            indegs[next] -= 1;
            if indegs[next] == 0 {
                queue.push_back(next);
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
        assert_eq!(
            sort_items(
                8,
                2,
                &mut [-1, -1, 1, 0, 0, 1, 0, -1],
                &[&[], &[6], &[5], &[6], &[3, 6], &[], &[], &[]]
            ),
            [6, 3, 4, 5, 2, 0, 7, 1]
        );
        assert!(sort_items(
            8,
            2,
            &mut [-1, -1, 1, 0, 0, 1, 0, -1],
            &[&[], &[6], &[5], &[6], &[3, 6], &[], &[4], &[]]
        )
        .is_empty());
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
