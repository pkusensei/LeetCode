mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_frequent_i_ds(nums: &[i32], freq: &[i32]) -> Vec<i64> {
    let max_val = *nums.iter().max().unwrap();
    let mut tree = SegmentTree::new(1 + max_val as usize);
    let mut res = vec![];
    for (&num, &f) in nums.iter().zip(freq.iter()) {
        tree.update(0, 0, max_val as usize, num as usize, f.into());
        res.push(tree.tree[0]);
    }
    res
}

struct SegmentTree {
    tree: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 4 * n],
        }
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, delta: i64) {
        if left == right {
            self.tree[node] += delta;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self.update(2 * node + 1, left, mid, idx, delta);
        } else {
            self.update(2 * node + 2, 1 + mid, right, idx, delta);
        }
        self.tree[node] = self.tree[2 * node + 1].max(self.tree[2 * node + 2]);
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
        assert_eq!(
            most_frequent_i_ds(&[2, 3, 2, 1], &[3, 2, -3, 1]),
            [3, 3, 2, 2]
        );
        assert_eq!(most_frequent_i_ds(&[5, 5, 3], &[2, -2, 1]), [2, 0, 1]);
    }

    #[test]
    fn test() {
        assert_eq!(
            most_frequent_i_ds(
                &[
                    18, 3, 2, 19, 14, 14, 17, 2, 15, 16, 10, 12, 5, 19, 17, 9, 16, 14, 14, 17, 13,
                    1, 7, 13, 13, 14, 2, 19, 4, 8, 15, 18, 11, 14, 11, 6, 11, 5
                ],
                &[
                    3, 3, 5, 4, 1, -1, 3, 5, 2, 2, 4, 1, 4, -2, -3, 3, -2, 2, 4, 1, 1, 1, 5, -1, 2,
                    4, 4, -2, 2, 5, 1, 2, 3, -7, 2, 3, -3, 3
                ]
            ),
            [
                3, 3, 5, 5, 5, 5, 5, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14
            ]
        );
    }
}
