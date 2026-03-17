mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn knight_dialer(n: i32) -> i32 {
    const M: i32 = 1_000_000_007;
    const MOVES: [&[usize]; 10] = [
        &[4, 6],    // 0
        &[6, 8],    // 1
        &[7, 9],    // 2
        &[4, 8],    // 3
        &[0, 3, 9], // 4
        &[],        // 5
        &[0, 1, 7], // 6
        &[2, 6],    // 7
        &[1, 3],    // 8
        &[2, 4],    // 9
    ];
    let mut prev = [1; 10];
    for _ in 1..n {
        let mut curr = [0; 10];
        for (i, &val) in prev.iter().enumerate() {
            for &next in MOVES[i] {
                curr[next] = (curr[next] + val) % M;
            }
        }
        prev = curr;
    }
    prev.iter().fold(0, |acc, v| (acc + v) % M)
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
