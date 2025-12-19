mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    dsu.union(0, first_person as usize);
    meetings.sort_unstable_by_key(|m| m[2]);
    for ch in meetings.chunk_by(|a, b| a[2] == b[2]) {
        for m in ch {
            dsu.union(m[0] as usize, m[1] as usize);
        }
        for m in ch {
            if dsu.find(m[0] as usize) != dsu.find(0) {
                dsu.reset(m[0] as usize);
                dsu.reset(m[1] as usize);
            }
        }
    }
    let root = dsu.find(0);
    (0..n)
        .filter_map(|v| {
            if dsu.find(v) == root {
                Some(v as i32)
            } else {
                None
            }
        })
        .collect()
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }

    fn reset(&mut self, v: usize) {
        self.parent[v] = v;
        self.rank[v] = 0;
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
    fn basics() {
        assert_eq!(max_profit(&[4, 2, 8], &[-1, 0, 1], 2), 10);
    }

    #[test]
    fn test() {
        assert_eq!(max_profit(&[4, 7, 13], &[-1, -1, 0], 2), 9);
    }
}
