mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn equations_possible(equations: Vec<String>) -> bool {
    let mut dsu = DSU::new();
    for eq in equations.iter() {
        if let Some((a, b)) = eq.split_once("==") {
            let [a, b] = [a, b].map(|s| usize::from(s.as_bytes()[0] - b'a'));
            dsu.union(a, b);
        }
    }
    for eq in equations.iter() {
        if let Some((a, b)) = eq.split_once("!=") {
            let [a, b] = [a, b].map(|s| usize::from(s.as_bytes()[0] - b'a'));
            if dsu.find(a) == dsu.find(b) {
                return false;
            }
        }
    }
    true
}

struct DSU {
    parent: [usize; 26],
    rank: [i32; 26],
}

impl DSU {
    fn new() -> Self {
        let mut parent = [0; 26];
        for (i, v) in parent.iter_mut().enumerate() {
            *v = i
        }
        Self {
            parent,
            rank: [0; 26],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
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
