mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_subarrays(nums: &[i32]) -> i64 {
    let mut st: Vec<(i32, i64)> = vec![];
    let mut res = 0;
    for &num in nums.iter() {
        while st.last().is_some_and(|&(v, _count)| v < num) {
            st.pop();
        }
        if st.last().is_none_or(|&(v, _)| v > num) {
            st.push((num, 1));
        } else if let Some((_, count)) = st.last_mut() {
            *count += 1;
        }
        res += st.last().map(|&(_, count)| count).unwrap_or(0);
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
        assert_eq!(number_of_subarrays(&[1, 4, 3, 3, 2]), 6);
        assert_eq!(number_of_subarrays(&[3, 3, 3]), 6);
        assert_eq!(number_of_subarrays(&[1]), 1);
    }

    #[test]
    fn test() {}
}
