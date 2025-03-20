mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_gcd(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut tree = SegmentTree::new(n);
    for (idx, &num) in nums.iter().enumerate() {
        tree.update(1, 0, n - 1, idx, num);
    }
    tree.find(1, 0, n - 1, k)
}

struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 4 * n],
        }
    }

    fn update(&mut self, u: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.tree[u] = val;
            return;
        }
        let mid = left + (right - left) / 2;
        if idx <= mid {
            self.update(2 * u, left, mid, idx, val);
        } else {
            self.update(2 * u + 1, 1 + mid, right, idx, val);
        }
        self.tree[u] = gcd(self.tree[2 * u], self.tree[2 * u + 1])
    }

    fn find(&self, u: usize, left: usize, right: usize, k: i32) -> i32 {
        if left == right {
            return i32::from(self.tree[u] == k);
        }
        let mut res = i32::from(self.tree[u] == k);
        let mid = left + (right - left) / 2;
        res += self.find(2 * u, left, mid, k);
        res += self.find(2 * u + 1, 1 + mid, right, k);
        res
    }
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(subarray_gcd(&[9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(subarray_gcd(&[4], 7), 0);
    }

    #[test]
    fn test() {}
}
