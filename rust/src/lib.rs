mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn length_of_lis(nums: &[i32], k: i32) -> i32 {
    let n = *nums.iter().max().unwrap() as usize + 1;
    let mut tree = SegmentTree::new(n);
    let mut res = 1;
    for &num in nums.iter() {
        let temp = tree.find(1, 0, n - 1, [(num - k).max(0) as usize, num as usize - 1]);
        res = res.max(1 + temp);
        tree.update(1, 0, n - 1, num as usize, 1 + temp);
    }
    res
}

struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree {
            tree: vec![0; 4 * n],
        }
    }

    fn find(&self, u: usize, left: usize, right: usize, range: [usize; 2]) -> i32 {
        if right < range[0] || range[1] < left {
            return 0;
        }
        if left == right || (range[0] <= left && right <= range[1]) {
            return self.tree[u];
        }
        let mid = left + (right - left) / 2;
        let a = self.find(2 * u, left, mid, range);
        let b = self.find(2 * u + 1, 1 + mid, right, range);
        a.max(b)
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
        self.tree[u] = self.tree[2 * u].max(self.tree[2 * u + 1]);
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
        assert_eq!(length_of_lis(&[4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
        assert_eq!(length_of_lis(&[7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
        assert_eq!(length_of_lis(&[1, 5], 1), 1);
    }

    #[test]
    fn test() {}
}
