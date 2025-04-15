mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn sum_counts(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut last_pos = HashMap::new();
    let mut tree = SegmentTree::new(n);
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let start = last_pos.get(&num).map(|v| v + 1).unwrap_or(0);
        let end = idx;
        tree.add_one(0, 0, n - 1, start, end);
        res += tree.square[0];
        res %= MOD;
        last_pos.insert(num, idx);
    }
    res as i32
}

const MOD: i64 = 1_000_000_007;

struct SegmentTree {
    lazy: Vec<i64>,
    sum: Vec<i64>,
    square: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            lazy: vec![0; 4 * n],
            sum: vec![0; 4 * n],
            square: vec![0; 4 * n],
        }
    }

    fn update(&mut self, idx: usize, left: usize, right: usize) {
        if left < right {
            self.lazy[2 * idx + 1] += self.lazy[idx];
            self.lazy[2 * idx + 2] += self.lazy[idx];
        }
        let gap = 1 + right - left;
        let sum = self.sum[idx] + gap as i64 * self.lazy[idx];
        let square = self.square[idx]
            + 2 * self.lazy[idx] * self.sum[idx]
            + self.lazy[idx].pow(2) * gap as i64;
        self.sum[idx] = sum % MOD;
        self.square[idx] = square % MOD;
        self.lazy[idx] = 0;
    }

    fn add_one(&mut self, idx: usize, left: usize, right: usize, x: usize, y: usize) {
        self.update(idx, left, right);
        if right < x || left > y {
            return;
        }
        if x <= left && right <= y {
            self.lazy[idx] = 1;
            self.update(idx, left, right);
            return;
        }
        let mid = left.midpoint(right);
        self.add_one(2 * idx + 1, left, mid, x, y);
        self.add_one(2 * idx + 2, 1 + mid, right, x, y);
        self.sum[idx] = (self.sum[2 * idx + 2] + self.sum[2 * idx + 1]) % MOD;
        self.square[idx] = (self.square[2 * idx + 2] + self.square[2 * idx + 1]) % MOD;
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
        assert_eq!(sum_counts(&[1, 2, 1]), 15);
        assert_eq!(sum_counts(&[2, 2]), 3);
    }

    #[test]
    fn test() {}
}
