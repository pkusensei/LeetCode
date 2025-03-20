mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn product_queries(n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut powers = vec![];
    for bit in 0..=n.ilog2() {
        if (n >> bit) & 1 == 1 {
            powers.push(1_i64 << bit);
        }
    }
    queries
        .iter()
        .map(|q| {
            let [a, b] = [0, 1].map(|i| q[i] as usize);
            powers[a..=b]
                .iter()
                .fold(1, |acc, &v| (acc * v) % 1_000_000_007) as i32
        })
        .collect()
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
        assert_eq!(product_queries(15, &[[0, 1], [2, 2], [0, 3]]), [2, 4, 64]);
        assert_eq!(product_queries(2, &[[0, 0]]), [2]);
    }

    #[test]
    fn test() {}
}
