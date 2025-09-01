mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_beauty(nums: &[i32]) -> i32 {
    use std::collections::HashMap;
    let Some(&max) = nums.iter().max() else {
        return 0;
    };
    // Each number is divisible by itself
    let map = nums
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &num)| {
            acc.entry(num).or_default().push(i);
            acc
        });
    // Count number of strictly inc subseqs with certain gcd
    let mut count = vec![0; 1 + max as usize];
    // For each gcd candidate
    for gcd in 1..=max {
        // 1) Collect all indices where nums[i]%gcd==0
        let mut indices = vec![];
        for multiple in (gcd..=max).step_by(gcd as usize) {
            if let Some(v) = map.get(&multiple) {
                indices.extend_from_slice(v);
            }
        }
        // 1.1) This gcd candidate is invalid/has only one multiple(itself)
        if indices.len() <= 1 {
            count[gcd as usize] = indices.len() as i32;
            continue;
        }
        // 2) Use BIT to count strictly inc subseqs on current `gcd`
        indices.sort_unstable();
        // num_idx - position/rank in sorted order
        let ranks: HashMap<_, _> = indices
            .iter()
            .enumerate()
            .map(|(pos, &num_idx)| (num_idx, 1 + pos))
            .collect();
        let mut ft = FenwickTree::new(indices.len());
        // loop as gcd, 2*gcd, 3*gcd, ...
        for multiple in (gcd..=max).step_by(gcd as usize) {
            let Some(inds) = map.get(&multiple) else {
                continue;
            };
            for &i in inds.iter().rev() {
                let sorted_rank = ranks[&i];
                // subseqs ending on `i`
                // +1 for the current element
                let subseq_count = (1 + ft.query(sorted_rank - 1)) % M;
                count[gcd as usize] = (count[gcd as usize] + subseq_count) % M;
                ft.update(sorted_rank, subseq_count);
            }
        }
        // Example:
        // nums=[4,2,4,2] with indices [0,1,2,3]
        // For gcd=2: indices=[0,1,2,3], ranks={0->1, 1->2, 2->3, 3->4}

        // Processing multiple=2 (value 2): indices=[1,3] in reverse=[3,1]
        // - i=3, sorted_rank=4, ft.query(3)=0, subseq_count=1  # Single element [2]
        // - i=1, sorted_rank=2, ft.query(1)=0, subseq_count=1  # Single element [2]

        // Processing multiple=4 (value 4): indices=[0,2] in reverse=[2,0]
        // - i=2, sorted_rank=3, ft.query(2)=1, subseq_count=2  # Can extend: [2] -> [2,4] plus single [4]
        // - i=0, sorted_rank=1, ft.query(0)=0, subseq_count=1  # Single element [4]
    }
    // 3) Inclusion-Exclusion
    //    By subtracting all that's divisible by multiples of `gcd`
    //    it yields counts on exactly `gcd`
    for gcd in (1..=max).rev() {
        for multiple in (2 * gcd..=max).step_by(gcd as usize) {
            count[gcd as usize] = (count[gcd as usize] - count[multiple as usize]).rem_euclid(M);
        }
    }
    count
        .into_iter()
        .enumerate()
        .map(|(gcd, c)| gcd as i64 * i64::from(c) % i64::from(M))
        .fold(0, |acc, v| (acc + v) % i64::from(M)) as i32
}

const M: i32 = 1_000_000_007;

struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
        }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        let sz = self.tree.len();
        while idx < sz {
            self.tree[idx] = (self.tree[idx] + val) % M;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i32 {
        let mut res = 0;
        while idx > 0 {
            res = (res + self.tree[idx]) % M;
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
        assert_eq!(total_beauty(&[1, 2, 3]), 10);
        assert_eq!(total_beauty(&[4, 6]), 12);
    }

    #[test]
    fn test() {}
}
