mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_energy(_n: i32, brightness: i32, mut intervals: Vec<[i32; 2]>) -> i64 {
    let count = (brightness + 2) / 3;
    intervals.sort_unstable();
    let mut prev_end = 0;
    let mut time = 0;
    for int in intervals.iter() {
        let [start, end] = int[..] else {
            unreachable!()
        };
        if end < prev_end {
            continue;
        }
        time += i64::from(1 + end - prev_end.max(start));
        prev_end = 1 + end;
    }
    i64::from(count) * time
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
        assert_eq!(min_energy(4, 2, vec![[1, 3], [2, 4]]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_energy(15, 8, vec![[2, 4], [18, 20], [8, 20], [14, 14]]),
            48
        )
    }
}
