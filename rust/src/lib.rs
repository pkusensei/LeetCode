mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cycle_length_queries(_n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    queries.iter().map(|q| solve(q[0], q[1])).collect()
}

fn solve(a: i32, b: i32) -> i32 {
    let mut min = a.min(b);
    let mut max = a.max(b);
    let mut res = 0;
    while min != max {
        max /= 2;
        if max < min {
            std::mem::swap(&mut min, &mut max);
        }
        res += 1;
    }
    1 + res
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
            cycle_length_queries(3, &[[5, 3], [4, 7], [2, 3]]),
            [4, 5, 3]
        );
        assert_eq!(cycle_length_queries(2, &[[1, 2]]), [2]);
        assert_eq!(
            cycle_length_queries(
                5,
                &[
                    [17, 21],
                    [23, 5],
                    [15, 7],
                    [3, 21],
                    [31, 9],
                    [5, 15],
                    [11, 2],
                    [19, 7]
                ]
            ),
            [7, 3, 2, 6, 8, 6, 3, 7]
        );
    }

    #[test]
    fn test() {}
}
