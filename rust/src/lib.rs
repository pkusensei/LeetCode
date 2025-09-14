mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if n <= 4 {
        return res;
    }
    if n & 1 == 1 {
        for i in 0..n {
            res.push(vec![(2 * i) % n, (2 * i + 1) % n]);
        }
        for i in 0..n {
            res.push(vec![(2 * i + 1) % n, (2 * i) % n]);
        }
    } else {
        for i in (0..n).step_by(2) {
            res.push(vec![i, 1 + i]);
        }
        for i in (0..n).step_by(2) {
            res.push(vec![1 + i, i]);
        }
        for i in (1..n).step_by(2) {
            res.push(vec![i, (1 + i) % n]);
        }
        for i in (1..n).step_by(2) {
            res.push(vec![(1 + i) % n, i]);
        }
    }
    for d in 2..(1 + n) / 2 {
        let start = res.last().unwrap()[0] + 1;
        for i in start..start + n {
            res.push(vec![i % n, (i + d) % n]);
        }
        let start = (res.last().unwrap()[1] - 1 + n) % n;
        for i in start..start + n {
            res.push(vec![(i + d) % n, i % n]);
        }
    }
    if n & 1 == 0 {
        let start = (res.last().unwrap()[0] - 1 + n) % n;
        for i in start..start + n {
            res.push(vec![i % n, (i + n / 2) % n]);
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
    fn basics() {
        assert_eq!(generate_schedule(5), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test() {}
}
