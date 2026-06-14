mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn max_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    // coord compression
    let map = nums
        .iter()
        .copied()
        .sorted_unstable()
        .fold(HashMap::new(), |mut acc, v| {
            let len = acc.len();
            acc.entry(v).or_insert(len);
            acc
        });
    let mut res = 0;
    let unique_n = map.len();
    let mut st = SegTree::new(unique_n, true);
    for num in nums.iter() {
        res += i64::from(*num);
        let idx = map[num];
        st.update(idx, 1, *num);
    }
    for left in 0..n {
        let mut running = 0;
        let mut outside = st.clone();
        let mut inside = SegTree::new(unique_n, false);
        for right in left..n {
            running += i64::from(nums[right]);
            res = res.max(running);
            outside.update(map[&nums[right]], -1, nums[right]);
            inside.update(map[&nums[right]], 1, nums[right]);
            let mut low = 0;
            let mut high = (1 + right - left)
                .min(n - (1 + right - left))
                .min(k as usize) as i32;
            while low < high {
                let mid = low + (1 + high - low) / 2;
                let a = outside.query_num(mid as i32);
                let b = inside.query_num(mid as i32);
                if a > b {
                    low = mid;
                } else {
                    high = mid - 1
                }
            }
            let a = outside.query_sum(low as i32);
            let b = inside.query_sum(low as i32);
            res = res.max(running + a - b);
        }
    }
    res
}

#[derive(Clone)]
struct SegTree {
    tree: Vec<(i32, i32, i64)>,
    n: usize,
    dir: bool,
}

impl SegTree {
    fn new(n: usize, dir: bool) -> Self {
        Self {
            tree: vec![(0, 0, 0); 4 * n],
            n,
            dir,
        }
    }

    fn update(&mut self, idx: usize, delta_f: i32, val: i32) {
        self._update(1, 0, self.n - 1, idx, delta_f, val);
    }

    fn _update(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        idx: usize,
        delta_f: i32,
        val: i32,
    ) {
        if left == right {
            self.tree[node].0 += delta_f;
            self.tree[node].1 = val;
            self.tree[node].2 += i64::from(delta_f * val);
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, delta_f, val);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, delta_f, val);
        }
        let a = self.tree[2 * node];
        let b = self.tree[2 * node + 1];
        let f = a.0 + b.0;
        let v = a.2 + b.2;
        self.tree[node] = (f, 0, v);
    }

    fn query_sum(&self, k: i32) -> i64 {
        self._query_sum(1, 0, self.n - 1, k).1
    }

    // dir false - left
    //     true - right
    fn _query_sum(&self, node: usize, left: usize, right: usize, k: i32) -> (i32, i64) {
        if self.tree[node].0 <= k {
            return (self.tree[node].0, self.tree[node].2);
        }
        if left == right {
            return (k, i64::from(k) * i64::from(self.tree[node].1));
        }
        let mid = left.midpoint(right);
        let preferred_half = if self.dir {
            self._query_sum(2 * node + 1, 1 + mid, right, k)
        } else {
            self._query_sum(2 * node, left, mid, k)
        };
        let kk = k - preferred_half.0;
        let mut other_half = (0, 0);
        if kk > 0 {
            other_half = if self.dir {
                self._query_sum(2 * node, left, mid, kk)
            } else {
                self._query_sum(2 * node + 1, 1 + mid, right, kk)
            };
        }
        (
            preferred_half.0 + other_half.0,
            preferred_half.1 + other_half.1,
        )
    }

    fn query_num(&self, k: i32) -> i32 {
        self._query_num(1, 0, self.n - 1, k)
    }

    fn _query_num(&self, node: usize, left: usize, right: usize, k: i32) -> i32 {
        if left == right {
            return self.tree[node].1;
        }
        let mid = left.midpoint(right);
        if self.dir {
            let temp = self.tree[2 * node + 1].0;
            if k <= temp {
                self._query_num(2 * node + 1, 1 + mid, right, k)
            } else {
                self._query_num(2 * node, left, mid, k - temp)
            }
        } else {
            let temp = self.tree[2 * node].0;
            if k <= temp {
                self._query_num(2 * node, left, mid, k)
            } else {
                self._query_num(2 * node + 1, 1 + mid, right, k - temp)
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
        assert_eq!(max_sum(&[1, -1, 0, 2], 1), 3);
    }

    #[test]
    fn test() {
        assert_eq!(max_sum(&[21, -4, 46], 0), 63);
        assert_eq!(
            max_sum(
                &[
                    -40789, 45205, 6766, -23824, 12187, -50512, -81753, 24695, 78030, -3678,
                    -52999, -96649, 76726, 28882, 76185, -72430, 3905, 14834, 37012, 72634, 77345,
                    -77717, -68748, -62418, 3497, 5021, 82199, 54811, -73335, 94361, -55771,
                    -83037, -4656, -47811, -11230, -47503, 55965, 5981, -67819, -38955, 11333,
                    90106, 30838, -26989, 87038, 52546, 55621, -28442, 98260, 68776, 4919, -6652,
                    24735, 74765, 58116, 6022, 27017
                ],
                32
            ),
            1546333
        );
        assert_eq!(max_sum(&[-46762, 62676, 16146, -40874, 66143], 2), 144965);
        assert_eq!(max_sum(&[-40, 7, -3, -39, 18, 42], 1), 67);
    }
}
