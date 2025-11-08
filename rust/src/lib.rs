mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 1;
    let mut pref = Vec::with_capacity(n);
    let mut prev = i32::MIN;
    for &num in nums.iter() {
        if prev <= num {
            pref.push(1 + pref.last().unwrap_or(&0));
        } else {
            pref.push(1);
        }
        prev = num;
        res = res.max(*pref.last().unwrap())
    }
    prev = i32::MAX;
    let mut suf = Vec::with_capacity(n);
    for &num in nums.iter().rev() {
        if num <= prev {
            suf.push(1 + suf.last().unwrap_or(&0));
        } else {
            suf.push(1);
        }
        prev = num;
        res = res.max(*suf.last().unwrap())
    }
    suf.reverse();
    if n > 1 {
        res = res.max(1 + pref[n - 2]).max(1 + suf[1]);
    }
    for i in 1..n - 1 {
        res = res.max(1 + pref[i]).max(1 + suf[i]);
        if nums[i - 1] <= nums[1 + i] {
            res = res.max(1 + pref[i - 1] + suf[1 + i]);
        }
    }
    res
}

// struct FenwickTree {
//     tree: Vec<i64>,
//     n: usize,
// }

// impl FenwickTree {
//     fn new(n: usize) -> Self {
//         Self {
//             tree: vec![0; 1 + n],
//             n,
//         }
//     }

//     fn update(&mut self, mut idx: usize, val: i64) {
//         idx += 1;
//         while idx <= self.n {
//             self.tree[idx] += val;
//             idx += idx & idx.wrapping_neg();
//         }
//     }

//     fn query(&self, mut idx: usize) -> i64 {
//         idx += 1;
//         let mut res = 0;
//         while idx > 0 {
//             res += self.tree[idx];
//             idx -= idx & idx.wrapping_neg();
//         }
//         res
//     }
// }

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
        assert_eq!(longest_subarray(&[1, 2, 3, 1, 2]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(longest_subarray(&[3, -4, -2]), 3);
        assert_eq!(longest_subarray(&[2, -2]), 2);
    }
}
