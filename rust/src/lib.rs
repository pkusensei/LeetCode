mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32]) -> i32 {
    let mut res = 0;
    let mut st = vec![]; // an increasing stack
    for &num in nums.iter() {
        if num == 0 {
            st.clear(); // start new section
            continue;
        }
        while st.last().is_some_and(|&v| v > num) {
            st.pop(); // bigger values are cleared by this point
        }
        if st.last().is_some_and(|&v| v == num) {
            continue; // equal nums are cleared in the same batch
        }
        st.push(num);
        res += 1;
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
    fn basics() {
        assert_eq!(min_operations(&[3, 1, 2, 1]), 3);
    }

    #[test]
    fn test() {}
}
