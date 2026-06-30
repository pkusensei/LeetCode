mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
    let [mut min, mut max] = [-1; 2];
    let mut max_f = 0;
    let mut sum = 0;
    let mut mode = 0;
    let mut prefix = Vec::with_capacity(256);
    for (i, &v) in count.iter().enumerate() {
        if v > 0 {
            if min == -1 {
                min = i as i32
            }
            max = i as i32;
            if v > max_f {
                mode = i as i32;
                max_f = v;
            }
            sum += i64::from(v) * i as i64;
        }
        prefix.push(v + prefix.last().unwrap_or(&0));
    }
    let total = prefix[255];
    let mean = sum as f64 / f64::from(prefix[255]);
    let median;
    if total & 1 == 1 {
        let i = prefix.partition_point(|&v| v <= total / 2);
        median = i as f64
    } else {
        let a = prefix.partition_point(|&v| v < total / 2);
        let b = prefix.partition_point(|&v| v < total / 2 + 1);
        median = (a + b) as f64 / 2.0;
    }
    vec![min.into(), max.into(), mean, median, mode.into()]
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
