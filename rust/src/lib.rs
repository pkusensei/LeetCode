mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_remove_all(types: Vec<i32>, colors: Vec<i32>, group_size: Vec<i32>) -> bool {
    let mut st: Vec<(i32, i32, i32)> = vec![];
    for (&t, &c) in types.iter().zip(colors.iter()) {
        if group_size[t as usize] == 1 {
            continue;
        }
        if let Some((top_t, top_c, top_count)) = st.last_mut()
            && *top_t == t
            && *top_c == c
        {
            if *top_count == group_size[t as usize] - 1 {
                st.pop();
            } else {
                *top_count += 1;
            }
        } else {
            st.push((t, c, 1));
        }
    }
    st.is_empty()
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
