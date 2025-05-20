mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_zero_array(nums: &[i32], queries: &[[i32; 2]]) -> bool {
    let n = nums.len();
    let mut diff = vec![0; 1 + n];
    for q in queries.iter() {
        let [a, b] = [0, 1].map(|i| q[i] as usize);
        diff[a] += 1;
        diff[1 + b] -= 1;
    }
    for i in 1..=n {
        diff[i] += diff[i - 1];
    }
    nums.iter().zip(diff).all(|(&a, b)| a <= b)
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
        assert!(is_zero_array(&[1, 0, 1], &[[0, 2]]));
        assert!(!is_zero_array(&[4, 3, 2, 1], &[[1, 3], [0, 2]]));
    }

    #[test]
    fn test() {}
}
