mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_triplets(nums1: &[i32], nums2: &[i32]) -> i64 {
    let n = nums1.len();
    let mut pos2 = vec![0; n]; // num=>idx in nums2
    for (i, &num) in nums2.iter().enumerate() {
        pos2[num as usize] = i;
    }
    // map: idx in nums2 => idx in nums1
    let mut pos2_pos1_idx_map = vec![0; n];
    for (i, &num) in nums1.iter().enumerate() {
        pos2_pos1_idx_map[pos2[num as usize]] = i;
    }
    let mut tree = FenwickTree::new(n);
    let mut res = 0;
    for pos2 in 0..n {
        let pos1 = pos2_pos1_idx_map[pos2];
        let left = i64::from(tree.query(pos1));
        tree.update(pos1, 1);
        // Count of indices bigger than pos1 - (Bigger, but placed to the left of pos1)
        let right = (n - 1 - pos1) as i64 - (pos2 as i64 - left);
        res += left * right;
    }
    res
}

struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i32) {
        idx += 1;
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            // idx&(-idx) => least significant bit of idx
            idx += idx & (!idx + 1);
        }
    }

    fn query(&mut self, mut idx: usize) -> i32 {
        let mut res = 0;
        idx += 1;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & (!idx + 1);
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
        assert_eq!(good_triplets(&[2, 0, 1, 3], &[0, 1, 2, 3]), 1);
        assert_eq!(good_triplets(&[4, 0, 1, 3, 2], &[4, 1, 0, 2, 3]), 4);
    }

    #[test]
    fn test() {}
}
