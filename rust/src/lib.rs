mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn equations_possible(equations: &[&str]) -> bool {
    let mut dsu = DSU::new();
    for [a, b] in equations.iter().filter_map(|s| {
        s.split_once("==")
            .and_then(|(a, b)| [a, b].map(|v| usize::from(v.as_bytes()[0] - b'a')).into())
    }) {
        dsu.union(a, b);
    }
    for [a, b] in equations.iter().filter_map(|s| {
        s.split_once("!=")
            .and_then(|(a, b)| [a, b].map(|v| usize::from(v.as_bytes()[0] - b'a')).into())
    }) {
        if dsu.find(a) == dsu.find(b) {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
struct DSU {
    parent: [usize; 26],
    rank: [usize; 26],
}

impl DSU {
    fn new() -> Self {
        let mut parent = [0; 26];
        for (i, v) in parent.iter_mut().enumerate() {
            *v = i;
        }
        Self {
            parent,
            rank: [1; 26],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        if x == y {
            return;
        }
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
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!equations_possible(&["a==b", "b!=a"]));
        debug_assert!(equations_possible(&["b==a", "a==b"]));
    }

    #[test]
    fn test() {
        debug_assert!(equations_possible(&["a!=b", "b!=c", "c!=a"]));
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
