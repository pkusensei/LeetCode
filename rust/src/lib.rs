mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct LUPrefix {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            parent: (0..n).collect(),
            size: vec![0; n],
        }
    }

    fn upload(&mut self, video: i32) {
        let idx = video as usize - 1;
        self.size[idx] += 1;
        if idx > 0 && self.size[idx - 1] > 0 {
            self.union(idx - 1, idx);
        }
        if self.size.get(1 + idx).is_some_and(|&v| v > 0) {
            self.union(idx, 1 + idx);
        }
    }

    fn longest(&mut self) -> i32 {
        let root = self.find(0);
        self.size[root]
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.size[ry] += self.size[rx];
            self.parent[rx] = ry;
        } else {
            self.size[rx] += self.size[ry];
            self.parent[ry] = rx;
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
    fn basics() {}

    #[test]
    fn test() {}
}
