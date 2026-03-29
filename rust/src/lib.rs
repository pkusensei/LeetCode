mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_edges_added(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    let mut res = 0;
    for e in edges {
        let [x, y, w] = e[..] else { unreachable!() };
        if dsu.union(x as usize, y as usize, w) {
            res += 1;
        } else if dsu.xor[x as usize] ^ dsu.xor[y as usize] ^ w == 0 {
            res += 1;
        }
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    xor: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            xor: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            let root = self.find(self.parent[v]);
            self.xor[v] ^= self.xor[self.parent[v]];
            self.parent[v] = root;
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize, w: i32) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        self.parent[ry] = rx;
        self.xor[ry] = self.xor[y] ^ self.xor[x] ^ w;
        true
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}
