mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pref_max = vec![nums[0]; n];
    for i in 1..n {
        pref_max[i] = nums[i].max(pref_max[i - 1]);
    }
    let mut suf_min = vec![nums[n - 1]; n];
    for i in (0..n - 1).rev() {
        suf_min[i] = nums[i].min(suf_min[1 + i]);
    }
    let mut dsu = DSU::new(&nums);
    for i in 0..n - 1 {
        if pref_max[i] > suf_min[1 + i] {
            dsu.union(i, 1 + i);
        }
    }
    (0..n)
        .map(|i| {
            let root = dsu.find(i);
            nums[dsu.find(root)]
        })
        .collect()
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        Self {
            parent: (0..n).collect(),
            rank: nums.to_vec(),
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
        if self.rank[rx] >= self.rank[ry] {
            self.parent[ry] = rx;
        } else {
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
        assert_eq!(max_value(vec![2, 1, 3]), [2, 2, 3]);
        assert_eq!(max_value(vec![2, 3, 1]), [3, 3, 3]);
    }

    #[test]
    fn test() {
        assert_eq!(max_value(vec![11, 18, 11]), [11, 18, 18]);
        assert_eq!(max_value(vec![30, 21, 5, 35, 24]), [35; 5]);
        assert_eq!(max_value(vec![13, 4, 11]), [13, 13, 13]);
    }
}
