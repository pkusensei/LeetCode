mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 2 * n;
    if let Some(zero) = check(&nums) {
        let a = zero;
        let b = 2 + n - zero;
        res = res.min(a).min(b);
    }
    let reversed: Vec<i32> = nums.iter().rev().copied().collect();
    if let Some(rev_zero) = check(&reversed) {
        let zero = n - rev_zero - 1;
        let rotate = n - zero - 1;
        let a = 1 + rotate;
        let b = 1 + n - rotate;
        res = res.min(a).min(b);
    }
    if res < 2 * n { res as i32 } else { -1 }
}

fn check(nums: &[i32]) -> Option<usize> {
    let n = nums.len();
    let Some(zero) = nums.iter().position(|&v| v == 0) else {
        return None;
    };
    let mut sorted = true;
    for i in 0..n {
        if nums[(i + zero) % n] as usize != i {
            sorted = false;
            break;
        }
    }
    if sorted { Some(zero) } else { None }
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
