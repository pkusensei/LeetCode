mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn edge_score(edges: Vec<i32>) -> i32 {
    let n = edges.len();
    let mut score = vec![0; n];
    for (i, &e) in edges.iter().enumerate() {
        score[e as usize] += i;
    }
    let mut res = 0;
    let mut max = 0;
    for (i, s) in (0..).zip(score) {
        if s > max {
            max = s;
            res = i;
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
    fn basics() {}

    #[test]
    fn test() {}
}
