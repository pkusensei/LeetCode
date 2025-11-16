mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_stable_subarrays(nums: &[i32], queries: &[[i32; 2]]) -> Vec<i64> {
    let n = nums.len();
    let mut streak = 0_i64;
    let mut prev = i32::MIN;
    let mut prefix = Vec::with_capacity(n);
    let mut dsu = DSU::new(n);
    for (i, &num) in nums.iter().enumerate() {
        if prev > num {
            streak = 0;
        } else if i > 0 {
            dsu.union(i - 1, i);
        }
        prev = num;
        streak += 1;
        prefix.push(streak + prefix.last().unwrap_or(&0));
    }
    let mut res = Vec::with_capacity(queries.len());
    for q in queries {
        let [a, b] = [0, 1].map(|i| q[i] as usize);
        let left_root = dsu.find(a);
        let curr = if left_root == dsu.find(b) {
            let v = (b - a + 1) as i64;
            v * (1 + v) / 2
        } else if left_root < a {
            let mut curr = prefix[b];
            let max = *dsu.packs[left_root].last().unwrap();
            curr -= prefix[max];
            let i = dsu.packs[left_root].binary_search(&a).unwrap();
            let len = dsu.packs[left_root].len();
            curr += ((len - i) * (len - i + 1) / 2) as i64;
            curr
        } else {
            prefix[b] - if a > 0 { prefix[a - 1] } else { 0 }
        };
        res.push(curr);
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    packs: Vec<Vec<usize>>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            packs: (0..n).map(|v| vec![v]).collect(),
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
        let temp = self.packs[ry].clone();
        self.packs[rx].extend(temp);
        self.parent[ry] = rx;
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
        assert_eq!(
            count_stable_subarrays(&[3, 1, 2], &[[0, 1], [1, 2], [0, 2]]),
            [2, 3, 4]
        );
        assert_eq!(count_stable_subarrays(&[2, 2], &[[0, 1], [0, 0]]), [3, 1]);
    }

    #[test]
    fn test() {
        assert_eq!(count_stable_subarrays(&[1, 2, 3], &[[1, 2]]), [3]);
        assert_eq!(count_stable_subarrays(&[8, 12], &[[1, 1]]), [1]);
    }
}
