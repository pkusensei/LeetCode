mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_val(n: i32, restrictions: Vec<[i32; 2]>, diff: &[i32]) -> i32 {
    let n = n as usize;
    let mut arr = vec![i32::MAX >> 1; n];
    arr[0] = 0;
    for r in restrictions {
        let [i, v] = r[..] else { unreachable!() };
        arr[i as usize] = arr[i as usize].min(v);
    }
    for (i, d) in diff.iter().enumerate() {
        arr[1 + i] = arr[1 + i].min(arr[i] + d);
    }
    for (i, d) in diff.iter().enumerate().rev() {
        arr[i] = arr[i].min(d + arr[1 + i]);
    }
    *arr.iter().max().unwrap()
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
            find_max_val(10, vec![[3, 1], [8, 1]], &[2, 2, 3, 1, 4, 5, 1, 1, 2]),
            6
        );
        assert_eq!(find_max_val(8, vec![[3, 2]], &[3, 5, 2, 4, 2, 3, 1]), 12);
    }

    #[test]
    fn test() {
        assert_eq!(find_max_val(3, vec![[2, 12]], &[5, 4]), 9);
        assert_eq!(find_max_val(2, vec![[1, 15]], &[2]), 2);
    }
}
