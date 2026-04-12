mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_subseq(mut nums: Vec<i32>, p: i32, queries: &[[i32; 2]]) -> i32 {
    let n = nums.len();
    let mut count = 0;
    for &num in nums.iter() {
        count += i32::from(num % p == 0);
    }
    let mut st = SegTree::new(&nums, p);
    let mut res = 0;
    for q in queries {
        let [idx, val] = q[..] else { unreachable!() };
        let idx = idx as usize;
        if nums[idx] % p == 0 {
            count -= 1
        }
        if val % p == 0 {
            count += 1;
        }
        nums[idx] = val;
        st.update(idx, val, p);
        // Ensures valid subseq
        if st.tree[1] == 1 {
            if count < n as i32 {
                res += 1;
            } else if st.can_skip() {
                res += 1;
            }
        }
    }
    res
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i32], p: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
        };
        s.build(nums, p, 1, 0, n - 1);
        s
    }

    fn build(&mut self, nums: &[i32], p: i32, node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = if nums[left] % p == 0 {
                nums[left] / p
            } else {
                0
            };
            return;
        }
        let mid = left.midpoint(right);
        self.build(nums, p, 2 * node, left, mid);
        self.build(nums, p, 2 * node + 1, 1 + mid, right);
        self.tree[node] = gcd(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    fn update(&mut self, idx: usize, val: i32, p: i32) {
        self._update(1, 0, self.n - 1, idx, val, p);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32, p: i32) {
        if left == right {
            self.tree[node] = if val % p == 0 { val / p } else { 0 };
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val, p);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, val, p);
        }
        self.tree[node] = gcd(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    fn can_skip(&self) -> bool {
        self._can_skip(1, 0, self.n - 1, 0)
    }

    fn _can_skip(&self, node: usize, left: usize, right: usize, val: i32) -> bool {
        if left == right {
            // Base case: external subtree already reduces to 1
            // This element can be skipped
            return val == 1;
        }
        let mid = left.midpoint(right);
        // Exclude this node/subtree from calculation
        let a = gcd(val, self.tree[2 * node + 1]);
        let b = gcd(val, self.tree[2 * node]);
        if a == 1 || b == 1 {
            return true;
        }
        self._can_skip(2 * node, left, mid, a) || self._can_skip(2 * node + 1, 1 + mid, right, b)
    }
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else if b == 0 {
        a // guard against either is 0
    } else {
        gcd(b % a, a)
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
            count_good_subseq(vec![4, 8, 12, 16], 2, &[[0, 3], [2, 6]]),
            1
        );
        assert_eq!(
            count_good_subseq(vec![4, 5, 7, 8], 3, &[[0, 6], [1, 9], [2, 3]]),
            2
        );
        assert_eq!(count_good_subseq(vec![5, 7, 9], 2, &[[1, 4], [2, 8]]), 0);
    }

    #[test]
    fn test() {}
}
