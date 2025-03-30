mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_reverse_operations(n: i32, p: i32, banned: &[i32], k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut dsu = DSU::new(2 + n as usize);
    dsu.union(p as usize, 2 + p as usize);
    for &b in banned.iter() {
        dsu.union(b as usize, 2 + b as usize);
    }
    let mut res = vec![-1; n as usize];
    res[p as usize] = 0;
    let mut queue = VecDeque::from([p]);
    let mut step = 0;
    while !queue.is_empty() {
        step += 1;
        let len = queue.len();
        for _ in 0..len {
            let i = queue.pop_front().unwrap();
            let min = (i - k + 1).max(k - i - 1);
            let max = (i + k - 1).min(2 * n - k - i - 1);
            let mut root = dsu.find(min as usize);
            // Banned idx would be in the same group/component (??)
            while root <= max as usize {
                res[root] = step;
                queue.push_back(root as i32);
                dsu.union(root, 2 + max as usize);
                root = dsu.find(2 + root);
            }
        }
    }
    res
}

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        // smh removing rank vec is the key
        if rx != ry {
            self.parent[rx] = ry;
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
        assert_eq!(min_reverse_operations(4, 0, &[1, 2], 4), [0, -1, -1, 1]);
        assert_eq!(min_reverse_operations(4, 2, &[0, 1, 3], 1), [-1, -1, 0, -1]);
        assert_eq!(
            min_reverse_operations(5, 0, &[2, 4], 3),
            [0, -1, -1, -1, -1]
        );
    }

    #[test]
    fn test() {
        assert_eq!(min_reverse_operations(4, 0, &[], 4), [0, -1, -1, 1]);
        assert_eq!(min_reverse_operations(5, 0, &[], 2), [0, 1, 2, 3, 4]);
    }
}
