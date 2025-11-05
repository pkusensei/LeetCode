mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn intersection_size_two(mut intervals: Vec<[i32; 2]>) -> i32 {
    intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(b[0].cmp(&a[0])));
    let mut res = 2;
    let mut right = intervals[0][1];
    let mut left = right - 1;
    for int in &intervals[1..] {
        let [a, b] = int[..] else { unreachable!() };
        if a <= left {
        } else if a <= right {
            left = right;
            right = b;
            res += 1;
        } else {
            right = b;
            left = right - 1;
            res += 2;
        }
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
    fn test() {
        assert_eq!(
            intersection_size_two(vec![[1, 3], [3, 7], [5, 7], [7, 8]]),
            5
        );
    }
}
