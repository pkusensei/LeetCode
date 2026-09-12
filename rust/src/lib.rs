mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distant_subarrays(nums: Vec<i32>, goal: i32, k: i32) -> i64 {
    let n = nums.len();
    if k == 0 {
        return (n * (1 + n) / 2) as i64;
    }
    let [goal, k] = [goal, k].map(i64::from);
    let prefix = nums.iter().fold(vec![0], |mut acc, &v| {
        acc.push(i64::from(v) + acc.last().unwrap_or(&0));
        acc
    });
    let mut sorted = prefix.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut ft = Fenwick::new(sorted.len());
    let mut res = 0;
    let mut total = 0;
    for &sum in &prefix {
        // goal-k ..= goal+k
        // sum-v >= goal+k
        // find all v <= sum-goal-k
        let i = sorted.partition_point(|&v| v <= sum - goal - k);
        res += ft.query(i);
        // sum - v <= goal-k
        // find all v >= sum-goal+k
        let i = sorted.partition_point(|&v| v < sum - goal + k);
        res += total - ft.query(i);
        let i = sorted.partition_point(|&v| v < sum);
        ft.update(1 + i);
        total += 1;
    }
    res as i64
}

struct Fenwick {
    tree: Vec<i64>,
    n: usize,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            n,
        }
    }

    fn update(&mut self, mut idx: usize) {
        while idx <= self.n {
            self.tree[idx] += 1;
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

    #[allow(unused_imports)]
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
    fn basics() {}

    #[test]
    fn test() {}
}
