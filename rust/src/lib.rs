mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn three_equal_parts(arr: &[i32]) -> Vec<i32> {
    use itertools::izip;
    let n = arr.len();
    let mut ones = arr.iter().sum::<i32>();
    if ones == 0 {
        return vec![0, n as i32 - 1];
    }
    if ones % 3 != 0 {
        return vec![-1, -1];
    }
    let mut ids = Vec::with_capacity(3);
    let mut f = 0;
    for (i, &v) in arr.iter().enumerate() {
        f += v;
        if f == ones / 3 {
            f = 0;
            ids.push(i);
        }
    }
    let zero = n - ids[2] - 1;
    let start1 = ids[0] + zero + 1;
    let start2 = ids[1] + zero + 1;
    for (a, b, c) in izip!(
        arr[..start1].iter().rev(),
        arr[start1..start2].iter().rev(),
        arr[start2..].iter().rev()
    ) {
        if a == b && b == c {
            ones -= 3 * a;
        } else {
            break;
        }
    }
    if ones == 0 {
        vec![start1 as i32 - 1, start2 as i32]
    } else {
        vec![-1, -1]
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(
            three_equal_parts(&[1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            [-1, -1]
        );
    }
}
