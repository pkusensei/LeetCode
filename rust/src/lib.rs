mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(n: i32, edges: &[[i32; 3]], query: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    for e in edges {
        dsu.union(e[0] as usize, e[1] as usize, e[2]);
    }
    query
        .iter()
        .map(|q| {
            let [a, b] = [0, 1].map(|i| dsu.find(q[i] as usize));
            if a == b { dsu.vals[a] } else { -1 }
        })
        .collect()
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    vals: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            vals: vec![2i32.pow(17) - 1; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize, val: i32) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            self.vals[rx] &= val;
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => {
                self.parent[rx] = ry;
                self.vals[ry] &= val;
                self.vals[ry] &= self.vals[rx];
            }
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
                self.vals[rx] &= val;
                self.vals[rx] &= self.vals[ry];
            }
            std::cmp::Ordering::Greater => {
                self.parent[ry] = rx;
                self.vals[rx] &= val;
                self.vals[rx] &= self.vals[ry];
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(
            minimum_cost(5, &[[0, 1, 7], [1, 3, 7], [1, 2, 1]], &[[0, 3], [3, 4]]),
            [1, -1]
        );
        assert_eq!(
            minimum_cost(3, &[[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]], &[[1, 2]]),
            [0]
        );
    }

    #[test]
    fn test() {}
}
