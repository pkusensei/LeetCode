mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_number_of_semesters(n: i32, relations: &[[i32; 2]], k: i32) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    let mut adj = vec![vec![]; n];
    let mut indegs = vec![0; n];
    for row in relations.iter() {
        let prev = row[0] as usize - 1;
        let next = row[1] as usize - 1;
        indegs[next] += 1;
        adj[prev].push(next);
    }
    dfs(&adj, k, indegs, (1 << n) - 1, &mut HashMap::new())
}

fn dfs(
    adj: &[Vec<usize>],
    k: usize,
    indegs: Vec<i32>,
    mask: u16,
    memo: &mut HashMap<(Vec<i32>, u16), i32>,
) -> i32 {
    if mask == 0 {
        return 0;
    }
    let _key = (indegs.clone(), mask);
    if let Some(&v) = memo.get(&_key) {
        return v;
    }
    let n = adj.len();
    let candids: Vec<_> = (0..n)
        .filter(|&v| mask & (1 << v) > 0 && indegs[v] == 0)
        .collect();
    let mut res = i32::MAX;
    for next in combinations(&candids, k.min(candids.len())) {
        let mut next_mask = mask;
        let mut indegs = indegs.clone();
        for &node in next.iter() {
            next_mask ^= 1 << node;
            for &c in adj[node].iter() {
                indegs[c] -= 1;
            }
        }
        res = res.min(1 + dfs(adj, k, indegs, next_mask, memo));
    }
    memo.insert(_key, res);
    res
}

fn combinations(elems: &[usize], k: usize) -> Vec<Vec<usize>> {
    fn backtrack(elems: &[usize], k: usize, curr: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
        if k == 0 {
            res.push(curr.clone());
            return;
        }
        for i in 0..elems.len() {
            curr.push(elems[i]);
            backtrack(&elems[1 + i..], k - 1, curr, res);
            curr.pop();
        }
    }
    let mut res = vec![];
    backtrack(elems, k, &mut vec![], &mut res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_number_of_semesters(4, &[[2, 1], [3, 1], [1, 4]], 2), 3);
        assert_eq!(
            min_number_of_semesters(5, &[[2, 1], [3, 1], [4, 1], [1, 5]], 2),
            4
        );
    }

    #[test]
    fn test() {
        assert_eq!(min_number_of_semesters(5, &[[3, 1]], 4), 2);
        assert_eq!(min_number_of_semesters(11, &[], 2), 6);
    }

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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
