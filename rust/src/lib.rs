mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let max = *nums.iter().max().unwrap() as usize;
    let freq = nums.iter().fold(vec![0; 1 + max], |mut acc, &num| {
        acc[num as usize] += 1;
        acc
    });
    let mut gcd_freq = vec![0_i64; 1 + max];
    for div in (1..=max).rev() {
        let mut count = 0;
        for v in (div..=max).step_by(div) {
            count += i64::from(freq[v]);
        }
        gcd_freq[div] += count * (count - 1) / 2;
        for v in (2 * div..=max).step_by(div) {
            gcd_freq[div] -= gcd_freq[v];
        }
    }
    for i in 1..=max {
        gcd_freq[i] += gcd_freq[i - 1];
    }
    let mut res = vec![];
    for &q in queries.iter() {
        let i = gcd_freq.partition_point(|&v| v <= q);
        res.push(i as i32);
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
    fn basics() {}

    #[test]
    fn test() {}
}
