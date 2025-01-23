mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// OR for point[r][c], rowcount[r]>1||colcount[c]>1
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut rmap = vec![vec![]; rows];
    let mut cmap = vec![vec![]; cols];
    let mut id = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                rmap[r].push(id);
                cmap[c].push(id);
                id += 1;
            }
        }
    }
    let mut dsu = DSU::new(id);
    for row in rmap {
        for w in row.windows(2) {
            dsu.union(w[0], w[1]);
        }
    }
    for col in cmap {
        for w in col.windows(2) {
            dsu.union(w[0], w[1]);
        }
    }
    (0..id)
        .filter(|&v| {
            let x = dsu.find(v);
            dsu.size[x] > 1
        })
        .count() as _
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.size[rx].cmp(&self.size[ry]) {
            std::cmp::Ordering::Less => {
                self.parent[rx] = ry;
                self.size[ry] += self.size[rx];
            }
            std::cmp::Ordering::Equal => {
                self.size[rx] += 1;
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
            }
            std::cmp::Ordering::Greater => {
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
            }
        }
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
