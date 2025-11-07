mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_chunks_to_sorted(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut left_max = Vec::with_capacity(n);
    let mut curr = 0;
    for &num in arr.iter() {
        curr = curr.max(num);
        left_max.push(curr);
    }
    curr = i32::MAX;
    let mut right_min = Vec::with_capacity(n);
    for &num in arr.iter().rev() {
        curr = curr.min(num);
        right_min.push(curr);
    }
    right_min.reverse();
    1 + left_max
        .iter()
        .zip(right_min.iter().skip(1))
        .filter(|(a, b)| a <= b)
        .count() as i32
}

pub fn with_stack(arr: &[i32]) -> i32 {
    let mut st = vec![];
    for &num in arr {
        if st.last().is_none_or(|&top| top <= num) {
            st.push(num);
        } else {
            let ele = *st.last().unwrap();
            while st.last().is_some_and(|&top| top > num) {
                st.pop();
            }
            st.push(ele);
        }
    }
    st.len() as i32
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
        assert_eq!(max_chunks_to_sorted(&[2, 1, 3, 4, 4]), 4);
        assert_eq!(with_stack(&[2, 1, 3, 4, 4]), 4);
    }

    #[test]
    fn test() {}
}
