mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_string_with_swaps(s: &str, pairs: &[[i32; 2]]) -> String {
    let n = s.len();
    let mut dsu = DSU::new(n);
    for p in pairs.iter() {
        // The idea is: [0, 1] and [1, 3] are swappable
        // Then [0, 1, 3] is a swappable group
        dsu.union(p[0] as usize, p[1] as usize);
    }
    let mut groups = vec![vec![]; n];
    for (i, b) in s.bytes().enumerate() {
        // Collect chars in a group together
        groups[dsu.find(i)].push(b);
    }
    for v in groups.iter_mut() {
        // As going from 0..n, pop smallest char first
        v.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    }
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        res.push(groups[dsu.find(i)].pop().unwrap());
    }
    String::from_utf8(res).unwrap()
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
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
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            smallest_string_with_swaps("dcab", &[[0, 3], [1, 2]]),
            "bacd"
        );
        assert_eq!(
            smallest_string_with_swaps("dcab", &[[0, 3], [1, 2], [0, 2]]),
            "abcd"
        );
        assert_eq!(smallest_string_with_swaps("cba", &[[0, 1], [1, 2]]), "abc");
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
