mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let max = *nums.iter().max().unwrap_or(&1) as usize;
    let mut gcd = vec![0_i64; 1 + max];
    let freq = nums.iter().fold(vec![0; 1 + max], |mut acc, &v| {
        acc[v as usize] += 1;
        acc
    });
    for div in (1..=max).rev() {
        let mut count = 0;
        for mult in (div..=max).step_by(div) {
            count += freq[mult]; // Every multiple of div
        }
        gcd[div] += count * (count - 1) / 2;
        for i in (2 * div..=max).step_by(div) {
            gcd[div] -= gcd[i]; // exclude double count
        }
    }
    for i in 1..=max as usize {
        gcd[i] += gcd[i - 1];
    }
    queries
        .iter()
        .map(|&q| gcd.partition_point(|&v| v <= q) as i32)
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
        assert_eq!(gcd_values(&[2, 3, 4], &[0, 2, 2]), [1, 2, 2]);
        assert_eq!(gcd_values(&[4, 4, 2, 1], &[5, 3, 1, 0]), [4, 2, 1, 1]);
    }

    #[test]
    fn test() {}
}
