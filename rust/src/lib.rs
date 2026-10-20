mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sortable_integers(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let fs = factors(n);
    if nums.is_sorted() {
        return fs.iter().sum::<usize>() as i32;
    }
    let mut res = 0;
    'out: for &k in &fs[1..] {
        let mut prev = 0;
        for w in nums.chunks(k) {
            if let Some(v) = is_sortable(w, prev) {
                prev = v;
            } else {
                continue 'out;
            }
        }
        res += k as i32
    }
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
