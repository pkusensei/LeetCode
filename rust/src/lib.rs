mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn sortable_integers(nums: &[i32]) -> i32 {
    let n = nums.len();
    let fs = factors(n);
    if nums.is_sorted() {
        return fs.iter().sum::<usize>() as i32;
    }
    let sorted = nums.iter().copied().sorted_unstable().collect_vec();
    let mut res = 0;
    'out: for &k in &fs[1..] {
        // let mut prev = 0;
        // for w in nums.chunks(k) {
        //     if let Some(v) = is_sortable(w, prev) {
        //         prev = v;
        //     } else {
        //         continue 'out;
        //     }
        // }
        for (sc, nc) in sorted.chunks(k).zip(nums.chunks(k)) {
            if !check(&sc, &nc) {
                continue 'out;
            }
        }
        res += k as i32
    }
    res
}

fn check(sorted: &[i32], nums: &[i32]) -> bool {
    let n = sorted.len();
    let mut arr = Vec::with_capacity(n);
    let min = sorted[0];
    let Some(i) = nums.iter().position(|&v| v == min) else {
        return false;
    };
    arr.extend_from_slice(&nums[i..]);
    arr.extend_from_slice(&nums[..i]);
    if arr == sorted {
        return true;
    }
    arr.clear();
    let max = sorted[n - 1];
    let Some(i) = nums.iter().rposition(|&v| v == max) else {
        return false;
    };
    arr.extend_from_slice(&nums[1 + i..]);
    arr.extend_from_slice(&nums[..=i]);
    sorted == arr
}

fn is_sortable(nums: &[i32], prev_min: i32) -> Option<i32> {
    let n = nums.len();
    let mut max = 0;
    let mut peak = false;
    for (i, &num) in nums.iter().enumerate() {
        if num < prev_min {
            return None;
        }
        max = max.max(num);
        if i < n - 1 && num > nums[1 + i] {
            if peak {
                return None;
            }
            peak = true
        }
    }
    if peak && nums[0] < nums[n - 1] {
        None
    } else {
        Some(max)
    }
}

fn factors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for f in 1..=n.isqrt() {
        if n % f == 0 {
            res.push(f);
            if f.pow(2) != n {
                res.push(n / f);
            }
        }
    }
    res.sort_unstable();
    res
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
    fn basics() {
        assert_eq!(sortable_integers(&[2, 1, 2]), 3);
    }

    #[test]
    fn test() {}
}
