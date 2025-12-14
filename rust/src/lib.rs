mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletions(s: &str, queries: &[&[i32]]) -> Vec<i32> {
    use itertools::Itertools;

    let n = s.len();
    let mut s = s.bytes().map(|b| b'B' - b).collect_vec();
    let mut ft = FenwickTree::new(n);
    for i in 1..n {
        // Fictional array eq[i] = (s[i]==s[i-1])
        ft.update(1 + i, i64::from(s[i] == s[i - 1])); // (2+i), s.windows(2)
    }
    let mut res = vec![];
    for q in queries {
        if q[0] == 1 {
            let idx = q[1] as usize;
            if let Some(&left) = idx.checked_sub(1).and_then(|i| s.get(i)) {
                if left == s[idx] {
                    ft.update(1 + idx, -1); // flip [i] to unequal
                } else {
                    ft.update(1 + idx, 1);
                }
            }
            if let Some(&right) = s.get(1 + idx) {
                if right == s[idx] {
                    ft.update(2 + idx, -1);
                } else {
                    ft.update(2 + idx, 1);
                }
            }
            s[idx] = 1 - s[idx];
        } else {
            // s[left..=right] => eq[(1+left)..=right] => ft[right]-ft[left]
            let curr = ft.query(1 + q[2] as usize) - ft.query(1 + q[1] as usize);
            res.push(curr as i32);
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

    fn update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&mut self, mut idx: usize) -> i64 {
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
            min_deletions("ABA", &[&[2, 1, 2], &[1, 1], &[2, 0, 2]]),
            [0, 2]
        );
        assert_eq!(
            min_deletions("ABB", &[&[2, 0, 2], &[1, 2], &[2, 0, 2]]),
            [1, 0]
        );
        assert_eq!(
            min_deletions("BABA", &[&[2, 0, 3], &[1, 1], &[2, 1, 3]]),
            [0, 1]
        );
    }

    #[test]
    fn test() {}
}
