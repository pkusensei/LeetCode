mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_effort(mut tasks: Vec<[i32; 2]>) -> i32 {
    // [actual, start]
    // sort_by(Reverse(start - actual))
    // sort_by(actual - start)
    tasks.sort_unstable_by_key(|v| v[0] - v[1]);
    let mut curr = tasks[0][1];
    let mut res = curr;
    for t in &tasks {
        let [actual, start] = t[..] else {
            unreachable!()
        };
        res += (start - curr).max(0);
        curr = curr.max(start) - actual;
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
    fn basics() {
        assert_eq!(
            minimum_effort(vec![[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]),
            32
        );
    }

    #[test]
    fn test() {}
}
