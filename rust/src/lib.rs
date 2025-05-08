mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let [mut even, mut odd, mut even_odd, mut odd_even] = [0; 4];
    for &num in &nums {
        let mut new_even_odd = even_odd;
        let mut new_odd_even = odd_even;
        if num & 1 == 0 {
            even += 1;
            new_odd_even = even_odd + 1;
        } else {
            odd += 1;
            new_even_odd = odd_even + 1;
        }
        even_odd = new_even_odd;
        odd_even = new_odd_even;
    }
    even.max(odd).max(even_odd).max(odd_even)
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
