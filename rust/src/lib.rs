mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarrays(n: i32, conflicting_pairs: &[[i32; 2]]) -> i64 {
    let n = n as usize;
    // cps[right] = [lefts..]
    let cps = conflicting_pairs
        .iter()
        .fold(vec![vec![]; 1 + n], |mut acc, c| {
            let [a, b] = [0, 1].map(|i| c[i] as usize);
            acc[a.max(b)].push(a.min(b));
            acc
        });
    let [mut rightmost_left, mut second_rightmost_left] = [0, 0];
    let mut res = 0;
    let mut regains = vec![0; 1 + n];
    for right in 1..=n {
        for &v in &cps[right] {
            if v > rightmost_left {
                second_rightmost_left = rightmost_left;
                rightmost_left = v
            } else if v > second_rightmost_left {
                second_rightmost_left = v
            }
        }
        // valid subarrs: [1+left..right]
        res += right - rightmost_left;
        // Remove rightmost_left to recover
        // [1+second_rightmost_left..right]
        // [2+second_rightmost_left..right]
        // ...
        regains[rightmost_left] += rightmost_left - second_rightmost_left;
    }
    (res + regains.into_iter().max().unwrap_or(0)) as i64
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
        assert_eq!(max_subarrays(4, &[[2, 3], [1, 4]]), 9);
        assert_eq!(max_subarrays(5, &[[1, 2], [2, 5], [3, 5]]), 12);
    }

    #[test]
    fn test() {}
}
