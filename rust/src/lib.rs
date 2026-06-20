mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
    restrictions.extend([vec![1, 0], vec![n, n - 1]]);
    restrictions.sort_unstable();
    let len = restrictions.len();
    let mut arr = vec![0; len];
    for i in 1..len {
        let dist = restrictions[i][0] - restrictions[i - 1][0];
        let v = arr[i - 1] + dist;
        arr[i] = v.min(restrictions[i][1]);
    }
    for i in (0..len - 1).rev() {
        let dist = restrictions[1 + i][0] - restrictions[i][0];
        let v = arr[1 + i] + dist;
        arr[i] = arr[i].min(v).min(restrictions[i][1]);
    }
    let mut res = 0;
    for i in 0..len - 1 {
        let a = arr[i];
        let b = arr[1 + i];
        let dist = restrictions[1 + i][0] - restrictions[i][0];
        let v = a.max(b) + (dist - (a - b).abs()) / 2;
        res = res.max(v);
    }
    res
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
