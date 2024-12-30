mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_connected(n: i32, connections: &[[i32; 2]]) -> i32 {
    let n = n as usize;
    if connections.len() + 1 < n {
        return -1;
    }
    let mut dsu = DSU::new(n);
    for e in connections.iter() {
        dsu.union(e[0] as usize, e[1] as usize);
    }
    dsu.count as i32 - 1
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(make_connected(4, &[[0, 1], [0, 2], [1, 2]]), 1);
        assert_eq!(
            make_connected(6, &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]),
            2
        );
        assert_eq!(make_connected(6, &[[0, 1], [0, 2], [0, 3], [1, 2]]), -1);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
