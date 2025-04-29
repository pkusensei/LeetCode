mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_value_sum(nums: &[i32], and_values: &[i32]) -> i32 {
    // let n1 = nums.len();
    // let n2 = and_values.len();
    // let mut dp = vec![vec![i32::MAX >> 1; 1 + n2]; 1 + n1];
    // dp[0][0] = 0;
    // let tree = SegmentTree::build(&nums);
    // for i2 in 1..=n2 {
    //     for i1 in 1..=n1 {
    //         let Some([a, b]) = search(&tree, and_values[i2 - 1], i1 - 1) else {
    //             continue;
    //         };
    //         for x in a..=b {
    //             dp[i1][i2] = dp[i1][i2].min(nums[i1 - 1] + dp[x][i2 - 1]);
    //         }
    //     }
    // }
    // if dp[n1][n2] >= i32::MAX >> 1 {
    //     -1
    // } else {
    //     dp[n1][n2]
    // }
    let res = dfs(nums, and_values, 0, 0, i32::MAX, &mut HashMap::new());
    if res >= INF { -1 } else { res }
}

const INF: i32 = i32::MAX >> 1;

fn dfs(
    nums: &[i32],
    and_values: &[i32],
    i1: usize,
    i2: usize,
    mut and_v: i32,
    memo: &mut HashMap<(usize, usize, i32), i32>,
) -> i32 {
    let n1 = nums.len();
    let n2 = and_values.len();
    if i1 >= n1 && i2 >= n2 {
        return 0;
    }
    if i1 >= n1 || i2 >= n2 {
        return INF;
    }
    let k = (i1, i2, and_v);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    and_v &= nums[i1];
    let res = match and_v.cmp(&and_values[i2]) {
        std::cmp::Ordering::Less => INF,
        std::cmp::Ordering::Equal => dfs(nums, and_values, 1 + i1, i2, and_v, memo)
            .min(nums[i1] + dfs(nums, and_values, 1 + i1, 1 + i2, i32::MAX, memo)),
        std::cmp::Ordering::Greater => dfs(nums, and_values, 1 + i1, i2, and_v, memo),
    };
    memo.insert(k, res);
    res
}

fn search(tree: &SegmentTree, and_v: i32, end: usize) -> Option<[usize; 2]> {
    let mut left = 0;
    let mut right = end;
    while left < right {
        let mid = left + (right - left) / 2;
        let v = tree.query_and(mid, end);
        if v >= and_v {
            let Some(i) = mid.checked_sub(1) else {
                break;
            };
            right = i;
        } else {
            left = mid + 1;
        }
    }
    let a = if tree.query_and(left, end) == and_v {
        left
    } else if tree.query_and(left + 1, end) == and_v {
        1 + left
    } else {
        return None;
    };
    left = 0;
    right = end;
    while left < right {
        let mid = left + (right - left) / 2;
        let v = tree.query_and(mid, end);
        if v <= and_v {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let b = if tree.query_and(left, end) == and_v {
        left
    } else if left > 0 && tree.query_and(left - 1, end) == and_v {
        left - 1
    } else {
        return None;
    };
    Some([a, b])
}

struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    pub fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
        };
        for (idx, &num) in nums.iter().enumerate() {
            s.update(0, 0, n - 1, idx, num);
        }
        s
    }

    pub fn query_and(&self, rl: usize, rr: usize) -> i32 {
        self.query_range(0, 0, self.n - 1, rl, rr)
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.tree[node] = val;
            return;
        }
        let mid = left + (right - left) / 2;
        if idx <= mid {
            self.update(2 * node + 1, left, mid, idx, val);
        } else {
            self.update(2 * node + 2, 1 + mid, right, idx, val);
        }
        self.tree[node] = self.tree[2 * node + 1] & self.tree[2 * node + 2];
    }

    fn query_range(&self, node: usize, left: usize, right: usize, rl: usize, rr: usize) -> i32 {
        if right < rl || rr < left {
            return i32::MAX;
        }
        if rl <= left && right <= rr {
            return self.tree[node];
        }
        let mid = left + (right - left) / 2;
        self.query_range(2 * node + 1, left, mid, rl, rr)
            & self.query_range(2 * node + 2, 1 + mid, right, rl, rr)
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
        assert_eq!(minimum_value_sum(&[1, 4, 3, 3, 2], &[0, 3, 3, 2]), 12);
        assert_eq!(minimum_value_sum(&[2, 3, 5, 7, 7, 7, 5], &[0, 7, 5]), 17);
        assert_eq!(minimum_value_sum(&[1, 2, 3, 4], &[2]), -1);
    }

    #[test]
    fn test() {}
}
