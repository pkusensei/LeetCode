mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_num_edges_to_remove(n: i32, edges: &[[i32; 3]]) -> i32 {
    let [type1, type2, type3] =
        edges
            .iter()
            .fold([const { vec![] }; 3], |[mut t1, mut t2, mut t3], e| {
                match e[0] {
                    1 => t1.push([e[1], e[2]]),
                    2 => t2.push([e[1], e[2]]),
                    _ => t3.push([e[1], e[2]]),
                };
                [t1, t2, t3]
            });
    let dsu = DSU::new(n as usize);
    let (dsu, v3) = connect(dsu, type3);
    let (d1, v1) = connect(dsu.clone(), type1);
    let (d2, v2) = connect(dsu, type2);
    if d1.n > 1 || d2.n > 1 {
        -1
    } else {
        v1 + v2 + v3
    }
}

fn connect(mut dsu: DSU, edges: Vec<[i32; 2]>) -> (DSU, i32) {
    let mut res = 0;
    for [x, y] in edges {
        if !dsu.union(x as usize - 1, y as usize - 1) {
            res += 1;
        }
    }
    (dsu, res)
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    n: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
            n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        self.n -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
