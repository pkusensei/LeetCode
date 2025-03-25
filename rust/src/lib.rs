mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn range_add_queries(n: i32, queries: &[[i32; 4]]) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = vec![vec![0; 1 + n]; n];
    for q in queries.iter() {
        let [r1, c1, r2, c2] = [0, 1, 2, 3].map(|i| q[i] as usize);
        for r in r1..=r2 {
            res[r][c1] += 1;
            res[r][1 + c2] -= 1;
        }
    }
    for r in 0..n {
        let mut curr = 0;
        for v in res[r].iter_mut() {
            curr += *v;
            *v = curr;
        }
        res[r].pop();
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
            range_add_queries(3, &[[1, 1, 2, 2], [0, 0, 1, 1]]),
            [[1, 1, 0], [1, 2, 1], [0, 1, 1]]
        );
    }

    #[test]
    fn test() {}
}
