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
    let map = nums
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &num)| {
            acc.entry(num).or_default().push(i);
            acc
        });
    let mut count = vec![0; 1 + max as usize];
    for div in 1..=max {
        let mut indices = vec![];
        for d in (div..=max).step_by(div as usize) {
            if let Some(v) = map.get(&d) {
                indices.extend_from_slice(v);
            }
        }
        if indices.len() <= 1 {
            count[div as usize] = indices.len() as i32;
            continue;
        }
        indices.sort_unstable();
        let rank: HashMap<_, _> = indices
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, 1 + i))
            .collect();
        let mut ft = FenwickTree::new(indices.len());
        for d in (div..=max).step_by(div as usize) {
            let Some(inds) = map.get(&d) else {
                continue;
            };
            for &i in inds.iter().rev() {
                let r = rank[&i];
                let add = (1 + ft.query(r - 1)) % M;
                count[div as usize] = (count[div as usize] + add) % M;
                ft.update(r, add);
            }
        }
    }
    for div in (1..=max).rev() {
        for e in (2 * div..=max).step_by(div as usize) {
            count[div as usize] = (count[div as usize] - count[e as usize]).rem_euclid(M);
        }
    }
    count
        .into_iter()
        .enumerate()
        .map(|(i, v)| i as i64 * i64::from(v) % i64::from(M))
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
