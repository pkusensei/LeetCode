mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximal_rectangle(matrix: &[&[i32]]) -> i32 {
    let cols = matrix[0].len();
    let mut res = 0;
    // Added 0 is used to pop all values in mono stack
    let mut height = vec![0; 1 + cols];
    for row in matrix {
        for (i, &c) in row.iter().enumerate() {
            if c == 1 {
                height[i] += 1;
            } else {
                height[i] = 0;
            }
        }
        let mut st = vec![];
        for (idx, &h) in height.iter().enumerate() {
            // Small values are the "thresholds" => Keep them in stack
            while let Some(&top) = st.last()
                && height[top] > h
            {
                // This "big" [top] is being processed
                st.pop();
                // `idx` is 1 beyond end [top] streak
                // `left` is 1 before start of [top] streak
                // len([top] streak) = idx-left-1
                let left = st.last().map(|&v| v as i32).unwrap_or(-1);
                res = res.max((idx as i32 - left - 1) * height[top]);
            }
            st.push(idx);
        }
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
        assert_eq!(
            maximal_rectangle(&[
                &[1, 0, 1, 0, 0],
                &[1, 0, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 0, 0, 1, 0]
            ]),
            6
        );
    }

    #[test]
    fn test() {}
}
