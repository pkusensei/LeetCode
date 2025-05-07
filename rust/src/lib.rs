mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_peaks(mut nums: Vec<i32>, queries: &[[i32; 3]]) -> Vec<i32> {
    let n = nums.len();
    let mut tree = FenwickTree::new(n);
    for i in 1..n - 1 {
        if nums[i - 1] < nums[i] && nums[i] > nums[1 + i] {
            tree.update(i, 1);
        }
    }
    let mut res = vec![];
    for q in queries {
        if q[0] == 1 {
            let [a, b] = [1, 2].map(|i| q[i] as usize);
            // exclude subarray boundaries
            let curr = tree.query(b.saturating_sub(1)) - tree.query(a);
            res.push(curr.max(0) as i32);
        } else {
            let idx = q[1] as usize;
            let val = q[2];
            if let Some(left) = idx.checked_sub(1) {
                if 0 < left && nums[left - 1] < nums[left] {
                    if nums[left] > nums[idx] && nums[left] <= val {
                        tree.update(left, -1);
                    } else if nums[left] <= nums[idx] && nums[left] > val {
                        tree.update(left, 1);
                    }
                }
            }
            if idx + 2 < n {
                let right = 1 + idx;
                if nums[right] > nums[1 + right] {
                    if nums[idx] >= nums[right] && val < nums[right] {
                        tree.update(right, 1);
                    } else if nums[idx] < nums[right] && val >= nums[right] {
                        tree.update(right, -1);
                    }
                }
            }
            if (1..n - 1).contains(&idx) {
                if (nums[idx - 1] < nums[idx] && nums[idx] > nums[1 + idx])
                    && !(nums[idx - 1] < val && val > nums[1 + idx])
                {
                    tree.update(idx, -1);
                } else if !(nums[idx - 1] < nums[idx] && nums[idx] > nums[1 + idx])
                    && (nums[idx - 1] < val && val > nums[1 + idx])
                {
                    tree.update(idx, 1);
                }
            }
            nums[idx] = val;
        }
    }
    res
}

struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            n,
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
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
            count_of_peaks(vec![3, 1, 4, 2, 5], &[[2, 3, 4], [1, 0, 4]]),
            [0]
        );
        assert_eq!(
            count_of_peaks(vec![4, 1, 4, 2, 1, 5], &[[2, 2, 4], [1, 0, 2], [1, 0, 4]]),
            [0, 1]
        );
    }

    #[test]
    fn test() {}
}
