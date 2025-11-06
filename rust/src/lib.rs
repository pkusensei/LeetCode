mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{BTreeSet, HashMap};
    let n = 1 + c as usize;
    let mut dsu = DSU::new(n);
    for con in connections {
        let [x, y] = con[..] else { unreachable!() };
        dsu.union(x as usize, y as usize);
    }
    let mut map = HashMap::<_, BTreeSet<_>>::new();
    for i in 1..n {
        let root = dsu.find(i);
        map.entry(root).or_default().insert(i as i32);
    }
    let mut res = vec![];
    for q in queries {
        let root = dsu.find(q[1] as usize);
        if q[0] == 1 {
            let Some(v) = map.get(&root) else {
                continue;
            };
            if v.contains(&q[1]) {
                res.push(q[1]);
            } else {
                res.push(*v.first().unwrap_or(&-1));
            }
        } else {
            let Some(v) = map.get_mut(&root) else {
                continue;
            };
            v.remove(&q[1]);
        }
    }
    res
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
                self.rank[rx] += 1;
                self.parent[ry] = rx;
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
