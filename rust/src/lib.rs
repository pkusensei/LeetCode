mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut left = n;
    for i in (0..n - 1).rev() {
        if arr[i] > arr[1 + i] {
            left = i;
            break;
        }
    }
    if left == n {
        return arr;
    }
    let mut right = 1 + left;
    for i in 1 + left..n {
        if arr[i] < arr[left] && arr[right] < arr[i] {
            right = i;
        }
    }
    arr.swap(left, right);
    arr
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
    fn basics() {}

    #[test]
    fn test() {}
}
