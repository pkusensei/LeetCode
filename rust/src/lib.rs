mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: &[[i32; 3]]) -> Vec<Vec<i32>> {
    let mut edges: Vec<_> = edges
        .iter()
        .enumerate()
        .map(|(i, e)| (i, e[0] as usize, e[1] as usize, e[2]))
        .collect();
    // idx a b weight
    edges.sort_by_key(|e| e.3);
    let n = n as usize;
    let weight = build_mst(n, &edges, None, None);
    let [mut c, mut p] = [0, 1].map(|_| vec![]);
    for e in edges.iter() {
        let exc = build_mst(n, &edges, None, Some(e));
        let inc = build_mst(n, &edges, Some(e), None);
        if exc > weight {
            c.push(e.0 as i32);
        } else if inc == weight {
            // else keyword is necessary
            p.push(e.0 as i32);
        }
    }
    vec![c, p]
}

fn build_mst(
    n: usize,
    edges: &[(usize, usize, usize, i32)],
    inc: Option<&(usize, usize, usize, i32)>,
    exc: Option<&(usize, usize, usize, i32)>,
) -> i32 {
    let mut dsu = DSU::new(n);
    let mut weight = 0;
    if let Some(e) = inc {
        dsu.union(e.1, e.2);
        weight += e.3;
    }
    for e in edges {
        if exc.is_some_and(|v| v.0 == e.0) {
            continue;
        }
        if dsu.union(e.1, e.2) {
            weight += e.3;
        }
    }
    if dsu.size == 1 {
        // forms a tree
        weight
    } else {
        i32::MAX
    }
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    ranks: Vec<i32>,
    size: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            ranks: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        match self.ranks[rx].cmp(&self.ranks[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.ranks[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        self.size -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                5,
                &[
                    [0, 1, 1],
                    [1, 2, 1],
                    [2, 3, 2],
                    [0, 3, 2],
                    [0, 4, 3],
                    [3, 4, 3],
                    [1, 4, 6]
                ]
            ),
            [vec![0, 1], vec![2, 3, 4, 5]]
        );
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                4,
                &[[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
            ),
            [vec![], vec![0, 1, 2, 3]]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                6,
                &[
                    [0, 1, 1],
                    [1, 2, 1],
                    [0, 2, 1],
                    [2, 3, 4],
                    [3, 4, 2],
                    [3, 5, 2],
                    [4, 5, 2]
                ]
            ),
            [vec![3], vec![0, 1, 2, 4, 5, 6]]
        );
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
