mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_trimmed_numbers(nums: &[&str], queries: &[[i32; 2]]) -> Vec<i32> {
    let qn = queries.len();
    // [idx in queries, kth, trim number]
    let mut queries: Vec<_> = queries
        .iter()
        .enumerate()
        .map(|(i, q)| [i, q[0] as usize, q[1] as usize])
        .collect();
    queries.sort_unstable_by(|a, b| a[2].cmp(&b[2]).then(a[1].cmp(&b[1])));
    let mut nums: Vec<_> = nums.iter().enumerate().map(|(i, s)| (i, *s)).collect();
    let mut res = vec![0; qn];
    // radix sort
    let mut exp = 0;
    let n = nums[0].1.len();
    for &q in queries.iter() {
        let [qi, k, trim] = q;
        while exp < trim {
            exp += 1;
            nums.sort_by_cached_key(|v| v.1.as_bytes()[n - exp]);
        }
        res[qi] = nums[k - 1].0 as i32;
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
            smallest_trimmed_numbers(
                &["102", "473", "251", "814"],
                &[[1, 1], [2, 3], [4, 2], [1, 2]]
            ),
            [2, 2, 1, 0]
        );
        assert_eq!(
            smallest_trimmed_numbers(&["24", "37", "96", "04"], &[[2, 1], [2, 2]]),
            [3, 0]
        )
    }

    #[test]
    fn test() {}
}
